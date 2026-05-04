use axum::{routing::get, Router};
use crate::error::AppError;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}

#[utoipa::path(
    get,
    path = "/healthz",
    tag = "health",
    responses(
        (status = 200, description = "Server is healthy", body = String),
    ),
)]
pub async fn healthz() -> &'static str {
    "OK"
}

#[utoipa::path(
    get,
    path = "/readyz",
    tag = "health",
    responses(
        (status = 200, description = "Server is ready (DB + Redis connected)", body = String),
        (status = 503, description = "Database or Redis not ready"),
    ),
)]
pub async fn readyz(axum::extract::State(state): axum::extract::State<AppState>) -> Result<&'static str, AppError> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|_| AppError::ServiceUnavailable("Database not ready".to_string()))?;

    let mut redis_conn = state.redis.clone();
    let _: String = redis::cmd("PING")
        .query_async(&mut redis_conn)
        .await
        .map_err(|_| AppError::ServiceUnavailable("Redis not ready".to_string()))?;

    Ok("OK")
}