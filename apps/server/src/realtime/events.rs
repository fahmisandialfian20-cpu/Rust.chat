use crate::domain::channel::Channel;
use crate::domain::message::Message;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum WsEvent {
    #[serde(rename = "hello")]
    Hello(HelloData),
    #[serde(rename = "message.created")]
    MessageCreated(MessageCreatedData),
    #[serde(rename = "message.edited")]
    MessageEdited(MessageEditedData),
    #[serde(rename = "message.deleted")]
    MessageDeleted(MessageDeletedData),
    #[serde(rename = "typing.updated")]
    TypingUpdated(TypingData),
    #[serde(rename = "presence.updated")]
    PresenceUpdated(PresenceData),
    #[serde(rename = "channel.created")]
    ChannelCreated(Channel),
    #[serde(rename = "channel.updated")]
    ChannelUpdated(Channel),
    #[serde(rename = "channel.deleted")]
    ChannelDeleted(Uuid),
    #[serde(rename = "channel.visibility_changed")]
    ChannelVisibilityChanged(Uuid),
    #[serde(rename = "error")]
    Error(ErrorData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HelloData {
    pub user_id: Uuid,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageCreatedData {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageEditedData {
    pub message_id: Uuid,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageDeletedData {
    pub message_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypingData {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub is_typing: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PresenceData {
    pub user_id: Uuid,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorData {
    pub code: String,
    pub message: String,
}

impl WsEvent {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
