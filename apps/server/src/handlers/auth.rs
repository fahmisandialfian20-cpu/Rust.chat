use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::user::{ClientInfo, User, ClientDevice};
use crate::error::AppError;
use crate::services::auth_service::AuthResponse;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct BootstrapRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub invite_code: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
    #[serde(default)]
    pub client: Option<ClientInfo>,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

pub async fn bootstrap_owner(
    State(state): State<AppState>,
    Json(payload): Json<BootstrapRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state
        .auth_service
        .bootstrap_owner(payload.username, payload.password)
        .await?;
    Ok(Json(response))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state
        .auth_service
        .register(payload.username, payload.password)
        .await?;
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state
        .auth_service
        .login(payload.username_or_email, payload.password, payload.client)
        .await?;
    Ok(Json(response))
}

pub async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<StatusCode, AppError> {
    state
        .auth_service
        .logout(auth_user.user_id_uuid()?, auth_user.session_id_uuid()?)
        .await?;
    Ok(StatusCode::OK)
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state
        .auth_service
        .refresh(payload.refresh_token)
        .await?;
    Ok(Json(response))
}

pub async fn me(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<User>, AppError> {
    let user = state
        .auth_service
        .get_current_user(auth_user.user_id_uuid()?)
        .await?;
    Ok(Json(user))
}

pub async fn list_devices(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<Vec<ClientDevice>>, AppError> {
    let devices = state
        .auth_service
        .list_devices(auth_user.user_id_uuid()?)
        .await?;
    Ok(Json(devices))
}

pub async fn revoke_device(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(device_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state
        .auth_service
        .revoke_device(device_id, auth_user.user_id_uuid()?)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post, delete};

    axum::Router::new()
        .route("/api/v1/auth/bootstrap-owner", post(bootstrap_owner))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/refresh", post(refresh))
        .route("/api/v1/auth/me", get(me))
        .route("/api/v1/auth/devices", get(list_devices))
        .route("/api/v1/auth/devices/{device_id}", delete(revoke_device))
}
