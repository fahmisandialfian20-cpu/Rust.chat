use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Invite {
    pub id: Uuid,
    pub code: String,
    pub space_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub created_by: Uuid,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub expires_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInvite {
    pub space_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub max_uses: Option<i32>,
    pub expires_at: Option<OffsetDateTime>,
}
