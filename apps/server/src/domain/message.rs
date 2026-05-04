use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub author_user_id: Uuid,
    pub content: String,
    pub kind: String,
    pub reply_to_message_id: Option<Uuid>,
    pub edited_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessage {
    pub content: String,
    pub kind: Option<String>,
    pub reply_to_message_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessage {
    pub content: Option<String>,
}