use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::role::{RoleWithPermissions, CreateRoleRequest, UpdateRoleRequest};
use crate::state::AppState;
use crate::error::AppError;

pub async fn list_roles(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
) -> Result<Json<Vec<RoleWithPermissions>>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let roles = state.role_service.list_roles(space_id, user_id).await?;
    Ok(Json(roles))
}

pub async fn create_role(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<Json<RoleWithPermissions>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let role = state.role_service.create_role(space_id, user_id, payload).await?;
    Ok(Json(role))
}

pub async fn get_role(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_space_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<RoleWithPermissions>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let role = state.role_service.get_role(role_id, user_id).await?;
    Ok(Json(role))
}

pub async fn update_role(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_space_id, role_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<RoleWithPermissions>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let role = state.role_service.update_role(role_id, user_id, payload).await?;
    Ok(Json(role))
}

pub async fn delete_role(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((_space_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    state.role_service.delete_role(role_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn assign_role_to_member(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((space_id, member_user_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<AssignRolePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let actor = auth_user.user_id_uuid()?;
    state.role_service.assign_role(space_id, member_user_id, payload.role_id, actor).await?;
    Ok(Json(serde_json::json!({"status": "assigned"})))
}

pub async fn remove_role_from_member(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((space_id, member_user_id, role_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let actor = auth_user.user_id_uuid()?;
    state.role_service.remove_role(space_id, member_user_id, role_id, actor).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(serde::Deserialize)]
pub struct AssignRolePayload {
    pub role_id: Uuid,
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post, put, delete};

    axum::Router::new()
        .route("/spaces/{space_id}/roles", get(list_roles))
        .route("/spaces/{space_id}/roles", post(create_role))
        .route("/spaces/{space_id}/roles/{role_id}", get(get_role))
        .route("/spaces/{space_id}/roles/{role_id}", put(update_role))
        .route("/spaces/{space_id}/roles/{role_id}", delete(delete_role))
        .route("/spaces/{space_id}/members/{user_id}/roles", post(assign_role_to_member))
        .route("/spaces/{space_id}/members/{user_id}/roles/{role_id}", delete(remove_role_from_member))
}
