use axum::{
    extract::{Path, State, Query},
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::auth::middleware::AuthUser;
use crate::domain::message::{Message, CreateMessage, UpdateMessage};
use crate::error::AppError;
use crate::middleware::rate_limit;
use crate::state::AppState;

#[derive(Deserialize, ToSchema)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    before: Option<Uuid>,
}

fn default_limit() -> i64 { 50 }

#[utoipa::path(
    post,
    path = "/api/v1/channels/{channel_id}/messages",
    tag = "messages",
    params(
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
    ),
    request_body = CreateMessage,
    responses(
        (status = 200, description = "Message created", body = Message),
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
pub async fn create_message(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<CreateMessage>,
) -> Result<Json<Message>, AppError> {
    let user_id = auth_user.user_id_uuid()?;
    let key = rate_limit::message_key(&auth_user.user_id);
    state
        .rate_limiter
        .check(&key, state.config.rate_limit.message_send, 60)
        .await?;
    let message = state
        .message_service
        .create_message(channel_id, user_id, payload)
        .await?;
    Ok(Json(message))
}

#[utoipa::path(
    get,
    path = "/api/v1/channels/{channel_id}/messages/{message_id}",
    tag = "messages",
    params(
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
        ("message_id" = Uuid, Path, description = "Message UUID"),
    ),
    responses(
        (status = 200, description = "Message found", body = Message),
        (status = 404, description = "Message not found"),
    ),
)]
pub async fn get_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Message>, AppError> {
    let message = state.message_service.get_message(message_id).await?;
    Ok(Json(message))
}

#[utoipa::path(
    get,
    path = "/api/v1/channels/{channel_id}/messages",
    tag = "messages",
    params(
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
        ("limit" = Option<i64>, Query, description = "Page limit"),
        ("before" = Option<Uuid>, Query, description = "Cursor: return messages before this ID"),
    ),
    responses(
        (status = 200, description = "List of messages", body = Vec<Message>),
    ),
)]
pub async fn list_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Message>>, AppError> {
    let messages = state.message_service.list_channel_messages(channel_id, query.limit, query.before).await?;
    Ok(Json(messages))
}

#[utoipa::path(
    put,
    path = "/api/v1/channels/{channel_id}/messages/{message_id}",
    tag = "messages",
    params(
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
        ("message_id" = Uuid, Path, description = "Message UUID"),
    ),
    request_body = UpdateMessage,
    responses(
        (status = 200, description = "Message updated", body = Message),
        (status = 404, description = "Message not found"),
    ),
)]
pub async fn update_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMessage>,
) -> Result<Json<Message>, AppError> {
    let message = state.message_service.update_message(message_id, Uuid::nil(), payload).await?;
    Ok(Json(message))
}

#[utoipa::path(
    delete,
    path = "/api/v1/channels/{channel_id}/messages/{message_id}",
    tag = "messages",
    params(
        ("channel_id" = Uuid, Path, description = "Channel UUID"),
        ("message_id" = Uuid, Path, description = "Message UUID"),
    ),
    responses(
        (status = 204, description = "Message deleted"),
        (status = 404, description = "Message not found"),
    ),
)]
pub async fn delete_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::http::StatusCode, AppError> {
    state.message_service.delete_message(message_id, Uuid::nil()).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub fn router() -> axum::Router<AppState> {
    use axum::routing::{get, post, put, delete};

    axum::Router::new()
        .route("/channels/{channel_id}/messages", post(create_message))
        .route("/channels/{channel_id}/messages", get(list_messages))
        .route("/channels/{channel_id}/messages/{message_id}", get(get_message))
        .route("/channels/{channel_id}/messages/{message_id}", put(update_message))
        .route("/channels/{channel_id}/messages/{message_id}", delete(delete_message))
}