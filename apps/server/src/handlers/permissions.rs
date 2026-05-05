use axum::{
    extract::{Path, State},
    response::Json,
};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::state::AppState;

pub async fn get_my_permissions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
) -> Result<Json<Vec<String>>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let permissions = state
        .permission_service
        .list_user_permissions(user_id, space_id)
        .await?;
    Ok(Json(permissions))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::get;
    axum::Router::new().route("/spaces/{space_id}/my-permissions", get(get_my_permissions))
}
