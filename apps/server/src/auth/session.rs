use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub status: SessionStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SessionStatus {
    Active,
    Revoked,
    Expired,
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::Active => write!(f, "active"),
            SessionStatus::Revoked => write!(f, "revoked"),
            SessionStatus::Expired => write!(f, "expired"),
        }
    }
}

pub struct SessionManager {
    redis_client: redis::Client,
    _db_pool: sqlx::PgPool,
}

impl SessionManager {
    pub fn new(redis_client: redis::Client, db_pool: sqlx::PgPool) -> Self {
        Self {
            redis_client,
            _db_pool: db_pool,
        }
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        token: String,
        refresh_token: Option<String>,
        ttl_seconds: i64,
    ) -> Result<Session, AppError> {
        let session_id = Uuid::now_v7();
        let now = Utc::now();
        let expires_at = now + Duration::seconds(ttl_seconds);

        let session = Session {
            id: session_id,
            user_id,
            token,
            refresh_token,
            expires_at,
            created_at: now,
            last_used_at: now,
            ip_address: None,
            user_agent: None,
            status: SessionStatus::Active,
        };

        self.store_session(&session).await?;
        Ok(session)
    }

    pub async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>, AppError> {
        let key = format!("session:{}", session_id);
        let mut conn = self
            .redis_client
            .get_connection_manager()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let data: Option<Vec<u8>> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if let Some(data) = data {
            if let Ok(session) = serde_json::from_slice::<Session>(&data) {
                if session.status == SessionStatus::Active && session.expires_at > Utc::now() {
                    return Ok(Some(session));
                }
            }
        }

        Ok(None)
    }

    pub async fn revoke_session(&self, session_id: Uuid) -> Result<(), AppError> {
        let key = format!("session:{}", session_id);
        let mut conn = self
            .redis_client
            .get_connection_manager()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let data: Option<Vec<u8>> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if let Some(data) = data {
            if let Ok(mut session) = serde_json::from_slice::<Session>(&data) {
                session.status = SessionStatus::Revoked;
                if let Ok(bytes) = serde_json::to_vec(&session) {
                    let _: () = redis::cmd("SET")
                        .arg(&key)
                        .arg(bytes)
                        .query_async(&mut conn)
                        .await
                        .map_err(|e| AppError::InternalServerError(e.to_string()))?;
                }
            }
        }

        Ok(())
    }

    async fn store_session(&self, session: &Session) -> Result<(), AppError> {
        let key = format!("session:{}", session.id);
        let bytes = serde_json::to_vec(session)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let ttl = session
            .expires_at
            .signed_duration_since(Utc::now())
            .num_seconds() as u64;

        let mut conn = self
            .redis_client
            .get_connection_manager()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(bytes)
            .arg("EX")
            .arg(ttl)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
