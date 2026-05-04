use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::membership::{AddMember, SpaceMembership};
use crate::domain::space::{CreateSpace, Space, UpdateSpace};
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
    path = "/api/v1/spaces",
    tag = "spaces",
    request_body = CreateSpace,
    responses(
        (status = 200, description = "Space created", body = Space),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn create_space(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<CreateSpace>,
) -> Result<Json<Space>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let space = state.space_service.create_space(user_id, payload).await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::SPACE_CREATE,
            user_id,
            Some(space.id),
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(Json(space))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
    ),
    responses(
        (status = 200, description = "Space found", body = Space),
        (status = 404, description = "Space not found"),
    ),
)]
pub async fn get_space(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.get_space(space_id).await?;
    Ok(Json(space))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/slug/{slug}",
    tag = "spaces",
    params(
        ("slug" = String, Path, description = "Space slug"),
    ),
    responses(
        (status = 200, description = "Space found", body = Space),
        (status = 404, description = "Space not found"),
    ),
)]
pub async fn get_space_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.get_space_by_slug(&slug).await?;
    Ok(Json(space))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces",
    tag = "spaces",
    params(
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "List of spaces", body = Vec<Space>),
    ),
)]
pub async fn list_spaces(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Space>>, AppError> {
    let spaces = state
        .space_service
        .list_spaces(query.limit, query.offset)
        .await?;
    Ok(Json(spaces))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/user/{user_id}",
    tag = "spaces",
    params(
        ("user_id" = Uuid, Path, description = "User UUID"),
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "User's spaces", body = Vec<Space>),
    ),
)]
pub async fn list_user_spaces(
    Path(user_id): Path<Uuid>,
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Space>>, AppError> {
    let spaces = state
        .space_service
        .list_user_spaces(user_id, query.limit, query.offset)
        .await?;
    Ok(Json(spaces))
}

#[utoipa::path(
    put,
    path = "/api/v1/spaces/{space_id}",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
    ),
    request_body = UpdateSpace,
    responses(
        (status = 200, description = "Space updated", body = Space),
        (status = 404, description = "Space not found"),
    ),
)]
pub async fn update_space(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<UpdateSpace>,
) -> Result<Json<Space>, AppError> {
    let space = state.space_service.update_space(space_id, payload).await?;
    Ok(Json(space))
}

#[utoipa::path(
    delete,
    path = "/api/v1/spaces/{space_id}",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
    ),
    responses(
        (status = 204, description = "Space deleted"),
        (status = 404, description = "Space not found"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn delete_space(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    state.space_service.delete_space(space_id).await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::SPACE_DELETE,
            user_id,
            Some(space_id),
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/spaces/{space_id}/members",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
    ),
    request_body = AddMember,
    responses(
        (status = 200, description = "Member added", body = SpaceMembership),
    ),
)]
pub async fn add_member(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<AddMember>,
) -> Result<Json<SpaceMembership>, AppError> {
    let membership = state.space_service.add_member(space_id, payload).await?;
    Ok(Json(membership))
}

#[utoipa::path(
    delete,
    path = "/api/v1/spaces/{space_id}/members/{user_id}",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("user_id" = Uuid, Path, description = "User UUID"),
    ),
    responses(
        (status = 204, description = "Member removed"),
        (status = 404, description = "Member not found"),
    ),
)]
pub async fn remove_member(
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.space_service.remove_member(space_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/members",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "List of members", body = Vec<SpaceMembership>),
    ),
)]
pub async fn list_members(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<SpaceMembership>>, AppError> {
    let members = state
        .space_service
        .get_members(space_id, query.limit, query.offset)
        .await?;
    Ok(Json(members))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/members/{user_id}",
    tag = "spaces",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("user_id" = Uuid, Path, description = "User UUID"),
    ),
    responses(
        (status = 200, description = "Member found", body = SpaceMembership),
        (status = 404, description = "Member not found"),
    ),
)]
pub async fn get_member(
    State(state): State<AppState>,
    Path((space_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<SpaceMembership>, AppError> {
    let member = state.space_service.get_member(space_id, user_id).await?;
    Ok(Json(member))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{delete, get, post, put};

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
        .route(
            "/spaces/{space_id}/members/{user_id}",
            delete(remove_member),
        )
}
