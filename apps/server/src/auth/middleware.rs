use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::auth::jwt::Claims;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub session_id: String,
}

impl AuthUser {
    pub fn user_id_uuid(&self) -> Result<Uuid, AppError> {
        Uuid::parse_str(&self.user_id).map_err(|e| AppError::Unauthorized(e.to_string()))
    }

    pub fn session_id_uuid(&self) -> Result<Uuid, AppError> {
        Uuid::parse_str(&self.session_id).map_err(|e| AppError::Unauthorized(e.to_string()))
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let claims = if let Some(claims) = parts.extensions.get::<Claims>() {
            claims.clone()
        } else {
            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.strip_prefix("Bearer "))
                .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

            let token_data = state.jwt_manager.verify_token(auth_header)?;
            token_data.claims
        };

        let session_id = Uuid::parse_str(&claims.session_id)
            .map_err(|_| AppError::Unauthorized("Invalid session".to_string()))?;

        let session = state
            .auth_service
            .session_manager_ref()
            .get_session(session_id)
            .await
            .map_err(|_| AppError::Unauthorized("Session error".to_string()))?;

        match session {
            Some(_) => Ok(AuthUser {
                user_id: claims.sub,
                session_id: claims.session_id,
            }),
            None => Err(AppError::Unauthorized("Session revoked or expired".to_string())),
        }
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: axum::http::Request<axum::body::Body>,
    next: Next,
) -> Response {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.strip_prefix("Bearer ").unwrap_or(s))
        .map(|s| s.to_string());

    let token = match auth_header {
        Some(t) => t,
        None => {
            return (StatusCode::UNAUTHORIZED, r#"{"error":"Missing authorization header"}"#).into_response();
        }
    };

    match state.jwt_manager.verify_token(&token) {
        Ok(token_data) => {
            request.extensions_mut().insert(token_data.claims);
            next.run(request).await
        }
        Err(e) => {
            (StatusCode::UNAUTHORIZED, format!(r#"{{"error":"{}"}}"#, e)).into_response()
        }
    }
}
