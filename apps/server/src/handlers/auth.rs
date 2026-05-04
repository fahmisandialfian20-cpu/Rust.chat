use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::user::{ClientDevice, ClientInfo, User};
use crate::error::AppError;
use crate::middleware::rate_limit;
use crate::services::auth_service::AuthResponse;
use crate::state::AppState;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct BootstrapRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub invite_code: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
    #[serde(default)]
    pub client: Option<ClientInfo>,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/bootstrap-owner",
    tag = "auth",
    request_body = BootstrapRequest,
    responses(
        (status = 200, description = "Owner account created", body = AuthResponse),
        (status = 409, description = "Instance already has an owner"),
    ),
)]
pub async fn bootstrap_owner(
    State(state): State<AppState>,
    Json(payload): Json<BootstrapRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state
        .auth_service
        .bootstrap_owner(payload.username, payload.password)
        .await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::BOOTSTRAP,
            response.user.id,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered", body = AuthResponse),
        (status = 409, description = "Username already taken"),
    ),
)]
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

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 429, description = "Too many login attempts"),
    ),
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let ip = "127.0.0.1".to_string();
    let key = rate_limit::login_key(&ip);
    state
        .rate_limiter
        .check(&key, state.config.rate_limit.login, 60)
        .await?;
    let response = state
        .auth_service
        .login(payload.username_or_email, payload.password, payload.client)
        .await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::LOGIN,
            response.user.id,
            None,
            None,
            None,
            None,
            None,
            Some(ip),
        )
        .await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "auth",
    responses(
        (status = 200, description = "Logged out successfully"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<StatusCode, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    state
        .auth_service
        .logout(user_id, auth_user.session_id_uuid()?)
        .await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::LOGOUT,
            user_id,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(StatusCode::OK)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Tokens refreshed", body = AuthResponse),
        (status = 401, description = "Invalid or expired refresh token"),
    ),
)]
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let response = state.auth_service.refresh(payload.refresh_token).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Current user info", body = User),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
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

#[utoipa::path(
    get,
    path = "/api/v1/auth/devices",
    tag = "auth",
    responses(
        (status = 200, description = "List of registered devices", body = Vec<ClientDevice>),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
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

#[utoipa::path(
    delete,
    path = "/api/v1/auth/devices/{device_id}",
    tag = "auth",
    params(
        ("device_id" = Uuid, Path, description = "Device ID"),
    ),
    responses(
        (status = 204, description = "Device revoked"),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("bearer_auth" = [])
),
)]
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
    use axum::routing::{delete, get, post};

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
