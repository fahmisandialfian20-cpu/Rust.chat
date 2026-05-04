use axum::{
    extract::{Path, State},
    response::Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::domain::channel::ChannelKind;
use crate::error::AppError;
use crate::permissions::keys::PermissionKey;
use crate::state::AppState;

#[derive(Deserialize, ToSchema)]
pub struct MediaTokenRequest {
    mode: String,
    intent: String,
    #[serde(default)]
    _client_type: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MediaTokenResponse {
    provider: String,
    url: String,
    room: String,
    token: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/channels/{channel_id}/media-token",
    tag = "media",
    params(
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    request_body = MediaTokenRequest,
    responses(
        (status = 200, description = "Media token created", body = MediaTokenResponse),
        (status = 400, description = "Invalid mode or intent"),
        (status = 403, description = "Forbidden"),
        (status = 503, description = "LiveKit not configured"),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn create_media_token(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<MediaTokenRequest>,
) -> Result<Json<MediaTokenResponse>, AppError> {
    if !state.config.livekit.enabled {
        return Err(AppError::ServiceUnavailable(
            "LiveKit is not configured".to_string(),
        ));
    }

    let user_id = auth_user.user_id_uuid()?;

    let mode = payload.mode.as_str();
    let intent = payload.intent.as_str();

    if mode != "voice" && mode != "video" {
        return Err(AppError::BadRequest(
            "mode must be 'voice' or 'video'".to_string(),
        ));
    }

    if intent != "join" && intent != "start" {
        return Err(AppError::BadRequest(
            "intent must be 'join' or 'start'".to_string(),
        ));
    }

    let channel = state.channel_service.get_channel(channel_id).await?;
    let space_id = channel.space_id;

    let expected_kind = match mode {
        "voice" => ChannelKind::Voice,
        "video" => ChannelKind::Video,
        _ => unreachable!(),
    };

    if channel.kind != expected_kind {
        return Err(AppError::BadRequest(format!(
            "Channel is a {} channel, not a {} channel",
            channel.kind, mode
        )));
    }

    let flags = state.channel_service.get_feature_flags(channel_id).await?;

    let feature_enabled = match mode {
        "voice" => flags.voice_group_enabled,
        "video" => flags.video_group_enabled,
        _ => unreachable!(),
    };

    if !feature_enabled {
        return Err(AppError::Forbidden(format!(
            "{} is not enabled in this channel",
            mode,
        )));
    }

    let permission = match (mode, intent) {
        ("voice", "join") => PermissionKey::JoinVoice,
        ("voice", "start") => PermissionKey::StartVoice,
        ("video", "join") => PermissionKey::JoinVideo,
        ("video", "start") => PermissionKey::StartVideo,
        _ => unreachable!(),
    };

    state
        .permission_service
        .check(user_id, permission, Some(space_id), Some(channel_id))
        .await?;

    let has_start_permission = if intent == "start" {
        let start_perm = match mode {
            "voice" => PermissionKey::StartVoice,
            "video" => PermissionKey::StartVideo,
            _ => unreachable!(),
        };
        state
            .permission_service
            .check_optional(user_id, start_perm, Some(space_id), Some(channel_id))
            .await?
    } else {
        false
    };

    let room_name = format!("space-{}-channel-{}", space_id, channel_id);
    let lk_config = &state.config.livekit;

    let jwt = livekit_api::access_token::AccessToken::with_api_key(
        &lk_config.api_key,
        &lk_config.api_secret,
    )
    .with_identity(&user_id.to_string())
    .with_ttl(std::time::Duration::from_secs(300))
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        room_admin: has_start_permission,
        ..Default::default()
    })
    .to_jwt()
    .map_err(|e| {
        AppError::InternalServerError(format!("Failed to generate LiveKit token: {}", e))
    })?;

    Ok(Json(MediaTokenResponse {
        provider: "livekit".to_string(),
        url: lk_config.url.clone(),
        room: room_name,
        token: jwt,
    }))
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::post;
    axum::Router::new().route(
        "/channels/{channel_id}/media-token",
        post(create_media_token),
    )
}
