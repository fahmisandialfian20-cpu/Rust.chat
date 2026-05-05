use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Role {
    pub id: Uuid,
    pub space_id: Uuid,
    pub name: String,
    pub is_default: bool,
    pub permissions: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRoleRequest {
    pub name: String,
    pub permission_keys: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub permission_keys: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleWithPermissions {
    pub role: Role,
    pub permission_keys: Vec<String>,
}
