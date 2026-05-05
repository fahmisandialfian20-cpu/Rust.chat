use crate::error::AppError;
use redis::aio::ConnectionManager;
use uuid::Uuid;

const TYPING_TTL_SECS: u64 = 5;

#[derive(Clone)]
pub struct TypingService {
    redis: ConnectionManager,
}

impl TypingService {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn set_typing(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let key = format!("typing:{}:{}", channel_id, user_id);

        redis::cmd("SET")
            .arg(&key)
            .arg("1")
            .arg("EX")
            .arg(TYPING_TTL_SECS)
            .query_async::<()>(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn stop_typing(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let key = format!("typing:{}:{}", channel_id, user_id);

        redis::cmd("DEL")
            .arg(&key)
            .query_async::<()>(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn is_typing(&self, channel_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let key = format!("typing:{}:{}", channel_id, user_id);

        let exists: bool = redis::cmd("EXISTS")
            .arg(&key)
            .query_async(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(exists)
    }

    pub async fn get_typing_users(&self, channel_id: Uuid) -> Result<Vec<Uuid>, AppError> {
        let pattern = format!("typing:{}:*", channel_id);

        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(&pattern)
            .query_async(&mut self.redis.clone())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let mut user_ids = Vec::new();
        for key in keys {
            if let Some(user_id_str) = key.split(':').next_back() {
                if let Ok(user_id) = user_id_str.parse::<Uuid>() {
                    user_ids.push(user_id);
                }
            }
        }

        Ok(user_ids)
    }
}
