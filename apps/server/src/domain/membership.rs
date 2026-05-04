use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpaceMembership {
    pub id: Uuid,
    pub space_id: Uuid,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub settings: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct AddMember {
    pub user_id: Uuid,
    pub nickname: Option<String>,
}