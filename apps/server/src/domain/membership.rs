use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct SpaceMembership {
    pub id: Uuid,
    pub space_id: Uuid,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub settings: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddMember {
    pub user_id: Uuid,
    pub nickname: Option<String>,
}
