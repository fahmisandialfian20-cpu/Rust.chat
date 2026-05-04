use redis::aio::ConnectionManager;
use uuid::Uuid;
use crate::error::AppError;

const PRESENCE_TTL_SECS: u64 = 300;

#[derive(Clone)]
pub struct PresenceService {
    redis: ConnectionManager,
}

impl PresenceService {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn set_online(&self, user_id: Uuid) -> Result<(), AppError> {
        let key = format!("presence:{}", user_id);

        redis::cmd("SET")
            .arg(&key)
            .arg("online")
            .arg("EX")
            .arg(PRESENCE_TTL_SECS)
            .query_async::<()>(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn set_offline(&self, user_id: Uuid) -> Result<(), AppError> {
        let key = format!("presence:{}", user_id);

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn is_online(&self, user_id: Uuid) -> Result<bool, AppError> {
        let key = format!("presence:{}", user_id);

        let exists: bool = redis::cmd("EXISTS")
            .arg(&key)
            .query_async(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(exists)
    }

    pub async fn get_online_users(&self, user_ids: Vec<Uuid>) -> Result<Vec<Uuid>, AppError> {
        let mut online = Vec::new();

        for user_id in user_ids {
            if self.is_online(user_id).await? {
                online.push(user_id);
            }
        }

        Ok(online)
    }

    pub async fn refresh_presence(&self, user_id: Uuid) -> Result<(), AppError> {
        self.set_online(user_id).await
    }
}
