use time::OffsetDateTime;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct AuditEntry {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub space_id: Option<Uuid>,
    pub action: String,
    pub target_user_id: Option<Uuid>,
    pub target_space_id: Option<Uuid>,
    pub target_channel_id: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: OffsetDateTime,
}
