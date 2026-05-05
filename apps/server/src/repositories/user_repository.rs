use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::user::{ClientDevice, User, UserStatus};
use crate::error::AppError;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn create(
        &self,
        username: String,
        email: Option<String>,
        password_hash: String,
    ) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, SqlxUser>(
            r#"
            INSERT INTO users (id, username, email, password_hash, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'active', $5, $5)
            RETURNING id, username, email, password_hash, status, created_at, updated_at
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, SqlxUser>(
            r#"
            SELECT id, username, email, password_hash, status, created_at, updated_at
            FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, SqlxUser>(
            r#"
            SELECT id, username, email, password_hash, status, created_at, updated_at
            FROM users WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, SqlxUser>(
            r#"
            SELECT id, username, email, password_hash, status, created_at, updated_at
            FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_username_or_email(&self, value: &str) -> Result<User, AppError> {
        let row = sqlx::query_as::<_, SqlxUser>(
            r#"
            SELECT id, username, email, password_hash, status, created_at, updated_at
            FROM users WHERE username = $1 OR email = $1
            "#,
        )
        .bind(value)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn update_status(&self, id: Uuid, status: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET status = $1, updated_at = $2 WHERE id = $3")
            .bind(status)
            .bind(OffsetDateTime::now_utc())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn check_username_exists(&self, username: &str) -> Result<bool, AppError> {
        let result =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
                .bind(username)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    pub async fn check_email_exists(&self, email: &str) -> Result<bool, AppError> {
        let result =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(email)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    pub async fn list_devices(&self, user_id: Uuid) -> Result<Vec<ClientDevice>, AppError> {
        let rows = sqlx::query_as::<_, SqlxClientDevice>(
            r#"
            SELECT id, user_id, client_type, platform, device_name, push_token, last_seen_at, created_at
            FROM client_devices
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn register_device(
        &self,
        user_id: Uuid,
        client_type: &str,
        platform: Option<String>,
        device_name: Option<String>,
    ) -> Result<ClientDevice, AppError> {
        let row = sqlx::query_as::<_, SqlxClientDevice>(
            r#"
            INSERT INTO client_devices (id, user_id, client_type, platform, device_name, last_seen_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $6)
            RETURNING id, user_id, client_type, platform, device_name, push_token, last_seen_at, created_at
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(user_id)
        .bind(client_type)
        .bind(&platform)
        .bind(&device_name)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn delete_device(&self, device_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM client_devices WHERE id = $1 AND user_id = $2")
            .bind(device_id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Device not found".to_string()));
        }

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct SqlxUser {
    id: Uuid,
    username: String,
    email: Option<String>,
    password_hash: String,
    status: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<SqlxUser> for User {
    fn from(row: SqlxUser) -> Self {
        User {
            id: row.id,
            username: row.username,
            email: row.email,
            status: row.status.parse().unwrap_or(UserStatus::Pending),
            password_hash: Some(row.password_hash),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SqlxClientDevice {
    id: Uuid,
    user_id: Uuid,
    client_type: String,
    platform: Option<String>,
    device_name: Option<String>,
    push_token: Option<String>,
    last_seen_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
}

impl From<SqlxClientDevice> for ClientDevice {
    fn from(row: SqlxClientDevice) -> Self {
        ClientDevice {
            id: row.id,
            user_id: row.user_id,
            client_type: row.client_type,
            platform: row.platform,
            device_name: row.device_name,
            push_token: row.push_token,
            last_seen_at: row.last_seen_at,
            created_at: row.created_at,
        }
    }
}
