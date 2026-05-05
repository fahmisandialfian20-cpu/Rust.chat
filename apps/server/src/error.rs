use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String, u64),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let retry_after = if let AppError::TooManyRequests(_, secs) = &self {
            Some(*secs)
        } else {
            None
        };

        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            AppError::TooManyRequests(msg, _) => (StatusCode::TOO_MANY_REQUESTS, msg),
        };

        let body = Json(json!({ "error": error_message }));
        let mut response = (status, body).into_response();
        if let Some(secs) = retry_after {
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, secs.to_string().parse().unwrap());
        }
        response
    }
}
