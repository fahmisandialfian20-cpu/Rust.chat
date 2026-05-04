use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMessage {
    pub content: String,
    pub kind: Option<String>,
    pub reply_to_message_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMessage {
    pub content: Option<String>,
}
