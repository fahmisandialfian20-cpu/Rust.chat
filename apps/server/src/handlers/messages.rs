use axum::{
    extract::{Path, State, Query},
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::message::{Message, CreateMessage, UpdateMessage};
use crate::state::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
    before: Option<Uuid>,
}

fn default_limit() -> i64 { 50 }

pub async fn create_message(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<CreateMessage>,
) -> Result<Json<Message>, AppError> {
    let message = state.message_service.create_message(channel_id, Uuid::nil(), payload).await?;
    Ok(Json(message))
}

pub async fn get_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Message>, AppError> {
    let message = state.message_service.get_message(message_id).await?;
    Ok(Json(message))
}

pub async fn list_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Message>>, AppError> {
    let messages = state.message_service.list_channel_messages(channel_id, query.limit, query.before).await?;
    Ok(Json(messages))
}

pub async fn update_message(
    State(state): State<AppState>,
    Path((_channel_id, message_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateMessage>,
) -> Result<Json<Message>, AppError> {
    let message = state.message_service.update_message(message_id, Uuid::nil(), payload).await?;
    Ok(Json(message))
}

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