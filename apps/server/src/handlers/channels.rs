use axum::{
    extract::{Path, State, Query},
    response::Json,
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::channel::{Channel, CreateChannel, UpdateChannel, ChannelFeatureFlags, ChannelFeatureFlagsUpdate};
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

pub async fn create_channel(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<CreateChannel>,
) -> Result<Json<Channel>, AppError> {
    let channel = state.channel_service.create_channel(space_id, Uuid::nil(), payload).await?;
    Ok(Json(channel))
}

pub async fn get_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Channel>, AppError> {
    let channel = state.channel_service.get_channel(channel_id).await?;
    Ok(Json(channel))
}

pub async fn get_channel_by_slug(
    State(state): State<AppState>,
    Path((space_id, slug)): Path<(Uuid, String)>,
) -> Result<Json<Channel>, AppError> {
    let channel = state.channel_service.get_channel_by_slug(space_id, &slug).await?;
    Ok(Json(channel))
}

pub async fn list_channels(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Channel>>, AppError> {
    let channels = state.channel_service.list_space_channels(space_id, query.limit, query.offset).await?;
    Ok(Json(channels))
}

pub async fn list_visible_channels(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Channel>>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let channels = state.channel_service.list_visible_channels(space_id, user_id, query.limit, query.offset).await?;
    Ok(Json(channels))
}

pub async fn update_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateChannel>,
) -> Result<Json<Channel>, AppError> {
    let channel = state.channel_service.update_channel(channel_id, payload).await?;
    Ok(Json(channel))
}

pub async fn archive_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.channel_service.archive_channel(channel_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.channel_service.delete_channel(channel_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_channel_feature_flags(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ChannelFeatureFlags>, AppError> {
    let flags = state.channel_service.get_feature_flags(channel_id).await?;
    Ok(Json(flags))
}

pub async fn update_channel_feature_flags(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<ChannelFeatureFlagsUpdate>,
) -> Result<Json<ChannelFeatureFlags>, AppError> {
    let flags = state.channel_service.update_feature_flags(channel_id, payload).await?;
    Ok(Json(flags))
}

pub async fn add_channel_member(
    State(state): State<AppState>,
    Path((_space_id, channel_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.channel_service.add_member(channel_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_channel_member(
    State(state): State<AppState>,
    Path((_space_id, channel_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.channel_service.remove_member(channel_id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post, put, delete};

    axum::Router::new()
        .route("/spaces/{space_id}/channels", post(create_channel))
        .route("/spaces/{space_id}/channels", get(list_channels))
        .route("/spaces/{space_id}/channels/visible", get(list_visible_channels))
        .route("/spaces/{space_id}/channels/{channel_id}", get(get_channel))
        .route("/spaces/{space_id}/channels/slug/{slug}", get(get_channel_by_slug))
        .route("/spaces/{space_id}/channels/{channel_id}", put(update_channel))
        .route("/spaces/{space_id}/channels/{channel_id}", delete(archive_channel))
        .route("/spaces/{space_id}/channels/{channel_id}/feature-flags", get(get_channel_feature_flags))
        .route("/spaces/{space_id}/channels/{channel_id}/feature-flags", put(update_channel_feature_flags))
        .route("/spaces/{space_id}/channels/{channel_id}/members/{user_id}", put(add_channel_member))
        .route("/spaces/{space_id}/channels/{channel_id}/members/{user_id}", delete(remove_channel_member))
}