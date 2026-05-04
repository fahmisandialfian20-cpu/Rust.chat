use axum::{
    extract::{Path, State, Query},
    response::Json,
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::space::{Space, CreateSpace, UpdateSpace};
use crate::domain::membership::{SpaceMembership, AddMember};
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

pub async fn create_space(
    State(state): State<AppState>,
    Json(payload): Json<CreateSpace>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.create_space(Uuid::nil(), payload).await?;
    Ok(Json(space))
}

pub async fn get_space(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.get_space(space_id).await?;
    Ok(Json(space))
}

pub async fn get_space_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.get_space_by_slug(&slug).await?;
    Ok(Json(space))
}

pub async fn list_spaces(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Space>>, AppError> {
    let spaces = state.space_service.list_spaces(query.limit, query.offset).await?;
    Ok(Json(spaces))
}

pub async fn list_user_spaces(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Space>>, AppError> {
    let spaces = state.space_service.list_user_spaces(user_id, query.limit, query.offset).await?;
    Ok(Json(spaces))
}

pub async fn update_space(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<UpdateSpace>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.update_space(space_id, payload).await?;
    Ok(Json(space))
}

pub async fn delete_space(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.space_service.delete_space(space_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_member(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<AddMember>,
) -> Result<Json<SpaceMembership>, AppError> {
    let membership = state.space_service.add_member(space_id, payload).await?;
    Ok(Json(membership))
}

pub async fn remove_member(
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.space_service.remove_member(space_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_members(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<SpaceMembership>>, AppError> {
    let members = state.space_service.get_members(space_id, query.limit, query.offset).await?;
    Ok(Json(members))
}

pub async fn get_member(
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<SpaceMembership>, AppError> {
    let member = state.space_service.get_member(space_id, user_id).await?;
    Ok(Json(member))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post, put, delete};

    axum::Router::new()
        .route("/spaces", post(create_space))
        .route("/spaces", get(list_spaces))
        .route("/spaces/{space_id}", get(get_space))
        .route("/spaces/slug/{slug}", get(get_space_by_slug))
        .route("/spaces/{space_id}", put(update_space))
        .route("/spaces/{space_id}", delete(delete_space))
        .route("/spaces/{space_id}/members", get(list_members))
        .route("/spaces/{space_id}/members", post(add_member))
        .route("/spaces/{space_id}/members/{user_id}", get(get_member))
        .route("/spaces/{space_id}/members/{user_id}", delete(remove_member))
}
