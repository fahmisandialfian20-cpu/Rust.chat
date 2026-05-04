use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::channel::{
    Channel, ChannelFeatureFlags, ChannelFeatureFlagsUpdate, CreateChannel, UpdateChannel,
};
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
    path = "/api/v1/spaces/{space_id}/channels",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
    ),
    request_body = CreateChannel,
    responses(
        (status = 200, description = "Channel created", body = Channel),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn create_channel(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
    Json(payload): Json<CreateChannel>,
) -> Result<Json<Channel>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let channel = state
        .channel_service
        .create_channel(space_id, user_id, payload)
        .await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::CHANNEL_CREATE,
            user_id,
            Some(space_id),
            None,
            None,
            Some(channel.id),
            None,
            None,
        )
        .await?;
    Ok(Json(channel))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    responses(
        (status = 200, description = "Channel found", body = Channel),
        (status = 404, description = "Channel not found"),
    ),
)]
pub async fn get_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Channel>, AppError> {
    let channel = state.channel_service.get_channel(channel_id).await?;
    Ok(Json(channel))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/channels/slug/{slug}",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("slug" = String, Path, description = "Channel slug"),
    ),
    responses(
        (status = 200, description = "Channel found", body = Channel),
        (status = 404, description = "Channel not found"),
    ),
)]
pub async fn get_channel_by_slug(
    State(state): State<AppState>,
    Path((space_id, slug)): Path<(Uuid, String)>,
) -> Result<Json<Channel>, AppError> {
    let channel = state
        .channel_service
        .get_channel_by_slug(space_id, &slug)
        .await?;
    Ok(Json(channel))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/channels",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "List of channels", body = Vec<Channel>),
    ),
)]
pub async fn list_channels(
    State(state): State<AppState>,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Channel>>, AppError> {
    let channels = state
        .channel_service
        .list_space_channels(space_id, query.limit, query.offset)
        .await?;
    Ok(Json(channels))
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/channels/visible",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("offset" = Option<i64>, Query, description = "Page offset"),
    ),
    responses(
        (status = 200, description = "Visible channels", body = Vec<Channel>),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn list_visible_channels(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(space_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Channel>>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let channels = state
        .channel_service
        .list_visible_channels(space_id, user_id, query.limit, query.offset)
        .await?;
    Ok(Json(channels))
}

#[utoipa::path(
    put,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    request_body = UpdateChannel,
    responses(
        (status = 200, description = "Channel updated", body = Channel),
        (status = 404, description = "Channel not found"),
    ),
)]
pub async fn update_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateChannel>,
) -> Result<Json<Channel>, AppError> {
    let channel = state
        .channel_service
        .update_channel(channel_id, payload)
        .await?;
    Ok(Json(channel))
}

#[utoipa::path(
    delete,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    responses(
        (status = 204, description = "Channel archived"),
        (status = 404, description = "Channel not found"),
    ),
)]
pub async fn archive_channel(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state.channel_service.archive_channel(channel_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}/hard",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    responses(
        (status = 204, description = "Channel permanently deleted"),
        (status = 404, description = "Channel not found"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn delete_channel(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    state.channel_service.delete_channel(channel_id).await?;
    state
        .audit_service
        .log(
            crate::services::audit_service::CHANNEL_DELETE,
            user_id,
            Some(space_id),
            None,
            None,
            Some(channel_id),
            None,
            None,
        )
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}/feature-flags",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    responses(
        (status = 200, description = "Channel feature flags", body = ChannelFeatureFlags),
        (status = 404, description = "Channel not found"),
    ),
)]
pub async fn get_channel_feature_flags(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ChannelFeatureFlags>, AppError> {
    let flags = state.channel_service.get_feature_flags(channel_id).await?;
    Ok(Json(flags))
}

#[utoipa::path(
    put,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}/feature-flags",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    request_body = ChannelFeatureFlagsUpdate,
    responses(
        (status = 200, description = "Feature flags updated", body = ChannelFeatureFlags),
        (status = 404, description = "Channel not found"),
    ),
)]
pub async fn update_channel_feature_flags(
    State(state): State<AppState>,
    Path((_space_id, channel_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<ChannelFeatureFlagsUpdate>,
) -> Result<Json<ChannelFeatureFlags>, AppError> {
    let flags = state
        .channel_service
        .update_feature_flags(channel_id, payload)
        .await?;
    Ok(Json(flags))
}

#[utoipa::path(
    put,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}/members/{user_id}",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
        ("user_id" = Uuid, Path, description = "User UUID"),
    ),
    responses(
        (status = 204, description = "Member added to channel"),
        (status = 404, description = "Channel or user not found"),
    ),
)]
pub async fn add_channel_member(
    State(state): State<AppState>,
    Path((_space_id, channel_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state
        .channel_service
        .add_member(channel_id, user_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "/api/v1/spaces/{space_id}/channels/{channel_id}/members/{user_id}",
    tag = "channels",
    params(
        ("space_id" = Uuid, Path, description = "Space UUID"),
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
        ("user_id" = Uuid, Path, description = "User UUID"),
    ),
    responses(
        (status = 204, description = "Member removed from channel"),
        (status = 404, description = "Channel or user not found"),
    ),
)]
pub async fn remove_channel_member(
    State(state): State<AppState>,
    Path((_space_id, channel_id, user_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    state
        .channel_service
        .remove_member(channel_id, user_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{delete, get, post, put};

    axum::Router::new()
        .route("/spaces/{space_id}/channels", post(create_channel))
        .route("/spaces/{space_id}/channels", get(list_channels))
        .route(
            "/spaces/{space_id}/channels/visible",
            get(list_visible_channels),
        )
        .route("/spaces/{space_id}/channels/{channel_id}", get(get_channel))
        .route(
            "/spaces/{space_id}/channels/slug/{slug}",
            get(get_channel_by_slug),
        )
        .route(
            "/spaces/{space_id}/channels/{channel_id}",
            put(update_channel),
        )
        .route(
            "/spaces/{space_id}/channels/{channel_id}",
            delete(archive_channel),
        )
        .route(
            "/spaces/{space_id}/channels/{channel_id}/feature-flags",
            get(get_channel_feature_flags),
        )
        .route(
            "/spaces/{space_id}/channels/{channel_id}/feature-flags",
            put(update_channel_feature_flags),
        )
        .route(
            "/spaces/{space_id}/channels/{channel_id}/members/{user_id}",
            put(add_channel_member),
        )
        .route(
            "/spaces/{space_id}/channels/{channel_id}/members/{user_id}",
            delete(remove_channel_member),
        )
}
