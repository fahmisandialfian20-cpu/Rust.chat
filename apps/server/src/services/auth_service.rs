use std::sync::Arc;
use uuid::Uuid;

use crate::auth::jwt::JwtManager;
use crate::auth::password::{hash_password, verify_password};
use crate::auth::session::SessionManager;
use crate::config::AppConfig;
use crate::domain::user::{User, ClientInfo, ClientDevice};
use crate::error::AppError;
use crate::repositories::user_repository::UserRepository;

#[derive(Debug, serde::Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthService {
    user_repo: Arc<UserRepository>,
    session_manager: Arc<SessionManager>,
    jwt_manager: JwtManager,
    config: AppConfig,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        session_manager: Arc<SessionManager>,
        jwt_manager: JwtManager,
        config: AppConfig,
    ) -> Self {
        Self {
            user_repo,
            session_manager,
            jwt_manager,
            config,
        }
    }

    pub async fn bootstrap_owner(
        &self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, AppError> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM instance_settings)",
        )
        .fetch_one(self.user_repo.pool())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if exists {
            return Err(AppError::Conflict(
                "Instance already has an owner".to_string(),
            ));
        }

        if username.len() < 3 {
            return Err(AppError::BadRequest(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.contains(' ') {
            return Err(AppError::BadRequest(
                "Username must not contain spaces".to_string(),
            ));
        }

        let password_hash = hash_password(&password, &self.config.auth.password_pepper)?;

        let user = self
            .user_repo
            .create(username, None, password_hash)
            .await?;

        sqlx::query(
            r#"
            INSERT INTO instance_settings (owner_user_id, instance_name)
            VALUES ($1, 'Rust Chat')
            "#,
        )
        .bind(user.id)
        .execute(self.user_repo.pool())
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        self.create_auth_response(user).await
    }

    pub async fn register(
        &self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, AppError> {
        if username.len() < 3 {
            return Err(AppError::BadRequest(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.contains(' ') {
            return Err(AppError::BadRequest(
                "Username must not contain spaces".to_string(),
            ));
        }

        if password.len() < 6 {
            return Err(AppError::BadRequest(
                "Password must be at least 6 characters".to_string(),
            ));
        }

        if self
            .user_repo
            .check_username_exists(&username)
            .await?
        {
            return Err(AppError::Conflict(
                "Username already taken".to_string(),
            ));
        }

        let password_hash = hash_password(&password, &self.config.auth.password_pepper)?;

        let user = self
            .user_repo
            .create(username, None, password_hash)
            .await?;

        self.create_auth_response(user).await
    }

    pub async fn login(
        &self,
        username_or_email: String,
        password: String,
        client_info: Option<ClientInfo>,
    ) -> Result<AuthResponse, AppError> {
        let user = self
            .user_repo
            .find_by_username_or_email(&username_or_email)
            .await?;

        let password_hash = user.password_hash.as_deref().unwrap_or("");
        let valid = verify_password(
            &password,
            &self.config.auth.password_pepper,
            password_hash,
        )?;

        if !valid {
            return Err(AppError::Unauthorized("Invalid password".to_string()));
        }

        if let Some(client) = client_info {
            self.user_repo
                .register_device(
                    user.id,
                    &client.client_type,
                    client.platform,
                    client.device_name,
                )
                .await?;
        }

        self.create_auth_response(user).await
    }

    pub async fn refresh(&self, refresh_token: String) -> Result<AuthResponse, AppError> {
        let token_data = self.jwt_manager.verify_token(&refresh_token)?;
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;
        let session_id = Uuid::parse_str(&token_data.claims.session_id)
            .map_err(|e| AppError::Unauthorized(e.to_string()))?;

        let session = self
            .session_manager
            .get_session(session_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Session not found or expired".to_string()))?;

        let user = self.user_repo.find_by_id(user_id).await?;

        let new_access_token = self
            .jwt_manager
            .create_access_token(&user.id.to_string(), &session.id.to_string())?;
        let new_refresh_token = self
            .jwt_manager
            .create_refresh_token(&user.id.to_string(), &session.id.to_string())?;

        Ok(AuthResponse {
            user: Self::strip_password_hash(user),
            access_token: new_access_token,
            refresh_token: new_refresh_token,
        })
    }

    pub async fn logout(
        &self,
        _user_id: Uuid,
        session_id: Uuid,
    ) -> Result<(), AppError> {
        self.session_manager.revoke_session(session_id).await
    }

    pub async fn get_current_user(&self, user_id: Uuid) -> Result<User, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;
        Ok(Self::strip_password_hash(user))
    }

    pub async fn list_devices(&self, user_id: Uuid) -> Result<Vec<ClientDevice>, AppError> {
        self.user_repo.list_devices(user_id).await
    }

    pub async fn revoke_device(
        &self,
        device_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        self.user_repo.delete_device(device_id, user_id).await
    }

    async fn create_auth_response(&self, user: User) -> Result<AuthResponse, AppError> {
        let session = self
            .session_manager
            .create_session(
                user.id,
                String::new(),
                None,
                self.config.auth.jwt_access_ttl_seconds,
            )
            .await?;

        let session_id = session.id.to_string();
        let access_token = self
            .jwt_manager
            .create_access_token(&user.id.to_string(), &session_id)?;
        let refresh_token = self
            .jwt_manager
            .create_refresh_token(&user.id.to_string(), &session_id)?;

        Ok(AuthResponse {
            user: Self::strip_password_hash(user),
            access_token,
            refresh_token,
        })
    }

    pub fn session_manager_ref(&self) -> &SessionManager {
        &self.session_manager
    }

    fn strip_password_hash(user: User) -> User {
        User {
            password_hash: None,
            ..user
        }
    }
}
