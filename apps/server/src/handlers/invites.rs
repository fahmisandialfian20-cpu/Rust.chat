use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::invite::{CreateInvite, Invite};
use crate::error::AppError;
use crate::state::AppState;

#[derive(Deserialize, ToSchema)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default = "default_offset")]
    offset: i64,
}

fn default_limit() -> i64 {
    50
}
fn default_offset() -> i64 {
    0
}

#[utoipa::path(
    post,
    path = "/api/v1/invites",
    tag = "invites",
    request_body = CreateInvite,
    responses(
        (status = 200, description = "Invite created", body = Invite),
    ),
)]
pub async fn create_invite(
    State(state): State<AppState>,
    Json(payload): Json<CreateInvite>,
) -> Result<Json<Invite>, AppError> {
    let invite = state
        .invite_service
        .create_invite(Uuid::nil(), payload)
        .await?;
    Ok(Json(invite))
}

#[utoipa::path(
    get,
    path = "/api/v1/invites/{invite_id}",
    tag = "invites",
    params(
        ("invite_id" = Uuid, Path, description = "Invite UUID"),
    ),
    responses(
        (status = 200, description = "Invite found", body = Invite),
        (status = 404, description = "Invite not found"),
    ),
)]
pub async fn get_invite(
    State(state): State<AppState>,
    Path(invite_id): Path<Uuid>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.get_invite(invite_id).await?;
    Ok(Json(invite))
}

#[utoipa::path(
    get,
    path = "/api/v1/invites/code/{code}",
    tag = "invites",
    params(
        ("code" = String, Path, description = "Invite code"),
    ),
    responses(
        (status = 200, description = "Invite found", body = Invite),
        (status = 404, description = "Invite not found"),
    ),
)]
pub async fn get_invite_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.get_invite_by_code(&code).await?;
    Ok(Json(invite))
}

#[utoipa::path(
    get,
    path = "/api/v1/invites/validate/{code}",
    tag = "invites",
    params(
        ("code" = String, Path, description = "Invite code"),
    ),
    responses(
        (status = 200, description = "Invite validated", body = Invite),
        (status = 404, description = "Invalid invite code"),
    ),
)]
pub async fn validate_invite(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.validate_invite(&code).await?;
    Ok(Json(invite))
}

#[utoipa::path(
    post,
    path = "/api/v1/invites/consume/{code}",
    tag = "invites",
    params(
        ("code" = String, Path, description = "Invite code"),
    ),
    responses(
        (status = 200, description = "Invite consumed", body = Invite),
        (status = 404, description = "Invalid invite code"),
    ),
)]
pub async fn consume_invite(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.consume_invite(&code).await?;
    Ok(Json(invite))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/invites",
    tag = "invites",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "List of space invites", body = Vec<Invite>),
    ),
)]
pub async fn list_space_invites(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Invite>>, AppError> {
    let invites = state
        .invite_service
        .list_space_invites(space_id, query.limit, query.offset)
        .await?;
    Ok(Json(invites))
}

#[utoipa::path(
    delete,
    path = "/api/v1/invites/{invite_id}",
    tag = "invites",
    params(
        ("invite_id" = Uuid, Path, description = "Invite UUID"),
    ),
    responses(
        (status = 204, description = "Invite deleted"),
        (status = 404, description = "Invite not found"),
    ),
)]
pub async fn delete_invite(
    State(state): State<AppState>,
    Path(invite_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.invite_service.delete_invite(invite_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/invites/{code}/accept",
    tag = "invites",
    params(
        ("code" = String, Path, description = "Invite code"),
    ),
    responses(
        (status = 200, description = "Invite accepted", body = String),
        (status = 400, description = "Invalid or expired invite"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn accept_invite(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(code): Path<String>,
) -> Result<Json<String>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let result = state.invite_service.accept_invite(&code, user_id).await?;
    Ok(Json(result))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{delete, get, post};

    axum::Router::new()
        .route("/invites", post(create_invite))
        .route("/invites/{invite_id}", get(get_invite))
        .route("/invites/code/{code}", get(get_invite_by_code))
        .route("/invites/validate/{code}", get(validate_invite))
        .route("/invites/consume/{code}", post(consume_invite))
        .route("/invites/{code}/accept", post(accept_invite))
        .route("/spaces/{space_id}/invites", get(list_space_invites))
        .route("/invites/{invite_id}", delete(delete_invite))
}
