use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileObject {
    pub id: Uuid,
    pub space_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub uploader_user_id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub storage_key: String,
    pub metadata: serde_json::Value,
    pub created_at: OffsetDateTime,
}
