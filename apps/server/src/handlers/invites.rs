use axum::{
    extract::{Path, State, Query},
    response::Json,
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::invite::{Invite, CreateInvite};
use crate::state::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default = "default_offset")]
    offset: i64,
}

fn default_limit() -> i64 { 50 }
fn default_offset() -> i64 { 0 }

pub async fn create_invite(
    State(state): State<AppState>,
    Json(payload): Json<CreateInvite>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.create_invite(Uuid::nil(), payload).await?;
    Ok(Json(invite))
}

pub async fn get_invite(
    State(state): State<AppState>,
    Path(invite_id): Path<Uuid>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.get_invite(invite_id).await?;
    Ok(Json(invite))
}

pub async fn get_invite_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.get_invite_by_code(&code).await?;
    Ok(Json(invite))
}

pub async fn validate_invite(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.validate_invite(&code).await?;
    Ok(Json(invite))
}

pub async fn consume_invite(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Json<Invite>, AppError> {
    let invite = state.invite_service.consume_invite(&code).await?;
    Ok(Json(invite))
}

pub async fn list_space_invites(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Invite>>, AppError> {
    let invites = state.invite_service.list_space_invites(space_id, query.limit, query.offset).await?;
    Ok(Json(invites))
}

pub async fn delete_invite(
    State(state): State<AppState>,
    Path(invite_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.invite_service.delete_invite(invite_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post, delete};

    axum::Router::new()
        .route("/invites", post(create_invite))
        .route("/invites/{invite_id}", get(get_invite))
        .route("/invites/code/{code}", get(get_invite_by_code))
        .route("/invites/validate/{code}", get(validate_invite))
        .route("/invites/consume/{code}", post(consume_invite))
        .route("/spaces/{space_id}/invites", get(list_space_invites))
        .route("/invites/{invite_id}", delete(delete_invite))
}