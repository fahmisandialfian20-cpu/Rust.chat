use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::audit::AuditEntry;
use crate::error::AppError;
use crate::permissions::keys::PermissionKey;
use crate::permissions::PermissionService;
use crate::repositories::audit_repository::AuditRepository;

pub const BOOTSTRAP: &str = "bootstrap";
pub const LOGIN: &str = "login";
pub const LOGOUT: &str = "logout";
pub const REGISTER: &str = "register";
pub const SPACE_CREATE: &str = "space_create";
pub const SPACE_DELETE: &str = "space_delete";
pub const CHANNEL_CREATE: &str = "channel_create";
pub const CHANNEL_DELETE: &str = "channel_delete";
pub const MEMBER_ADD: &str = "member_add";
pub const MEMBER_REMOVE: &str = "member_remove";
pub const ROLE_CHANGE: &str = "role_change";
pub const PERMISSION_CHANGE: &str = "permission_change";
pub const FILE_UPLOAD: &str = "file_upload";
pub const FILE_DELETE: &str = "file_delete";
pub const MESSAGE_DELETE: &str = "message_delete";

#[derive(Clone)]
pub struct AuditService {
    repo: AuditRepository,
    permission_service: PermissionService,
}

impl AuditService {
    pub fn new(repo: AuditRepository, permission_service: PermissionService) -> Self {
        Self {
            repo,
            permission_service,
        }
    }

    pub async fn log(
        &self,
        action: &str,
        actor_user_id: Uuid,
        space_id: Option<Uuid>,
        target_user_id: Option<Uuid>,
        target_space_id: Option<Uuid>,
        target_channel_id: Option<Uuid>,
        metadata: Option<serde_json::Value>,
        ip_address: Option<String>,
    ) -> Result<(), AppError> {
        let entry = AuditEntry {
            id: Uuid::now_v7(),
            user_id: Some(actor_user_id),
            space_id,
            action: action.to_string(),
            target_user_id,
            target_space_id,
            target_channel_id,
            metadata: metadata.unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
            ip_address,
            user_agent: None,
            created_at: OffsetDateTime::now_utc(),
        };
        self.repo.insert(&entry).await
    }

    pub async fn list_audit_logs(
        &self,
        requesting_user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditEntry>, AppError> {
        self.permission_service
            .check(
                requesting_user_id,
                PermissionKey::ViewAuditLog,
                None,
                None,
            )
            .await?;
        self.repo.find_all(limit, offset).await
    }
}
