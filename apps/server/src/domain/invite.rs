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

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct InviteResponse {
    pub id: Uuid,
    pub space_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub created_by: Uuid,
    pub max_uses: Option<i32>,
    pub used_count: i32,
    pub expires_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl From<Invite> for InviteResponse {
    fn from(invite: Invite) -> Self {
        Self {
            id: invite.id,
            space_id: invite.space_id,
            channel_id: invite.channel_id,
            created_by: invite.created_by,
            max_uses: invite.max_uses,
            used_count: invite.used_count,
            expires_at: invite.expires_at,
            created_at: invite.created_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateInvite {
    pub space_id: Option<Uuid>,
    pub channel_id: Option<Uuid>,
    pub max_uses: Option<i32>,
    pub expires_at: Option<OffsetDateTime>,
}
