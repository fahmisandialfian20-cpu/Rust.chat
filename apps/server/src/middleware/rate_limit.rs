use redis::aio::ConnectionManager;

use crate::error::AppError;

#[derive(Clone)]
pub struct RateLimiter {
    redis: ConnectionManager,
}

impl RateLimiter {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn check(&self, key: &str, max: u64, window_secs: u64) -> Result<(), AppError> {
        let mut conn = self.redis.clone();
        let count: u64 = redis::cmd("INCR")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Rate limit check failed: {}", e)))?;

        if count == 1 {
            let _: Result<(), _> = redis::cmd("EXPIRE")
                .arg(key)
                .arg(window_secs as i64)
                .query_async(&mut conn)
                .await;
        }

        if count > max {
            return Err(AppError::TooManyRequests(
                format!("Rate limit exceeded. Try again in {} seconds.", window_secs),
                window_secs,
            ));
        }

        Ok(())
    }
}

pub fn login_key(ip: &str) -> String {
    format!("ratelimit:login:{}", ip)
}

pub fn message_key(user_id: &str) -> String {
    format!("ratelimit:message:{}", user_id)
}

pub fn upload_key(user_id: &str) -> String {
    format!("ratelimit:upload:{}", user_id)
}
