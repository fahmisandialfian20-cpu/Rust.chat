use super::keys::PermissionKey;
use super::repository::PermissionRepository;
use crate::error::AppError;

#[derive(Clone)]
pub struct PermissionResolver {
    repo: PermissionRepository,
}

impl PermissionResolver {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            repo: PermissionRepository::new(pool),
        }
    }

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.repo.pool
    }

    pub async fn is_space_member(&self, user_id: Uuid, space_id: Uuid) -> Result<bool, AppError> {
        self.repo.is_space_member(space_id, user_id).await
    }

    pub async fn check(
        &self,
        user_id: Uuid,
        permission: PermissionKey,
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
    ) -> Result<PermissionResult, AppError> {
        let perm_key = permission.as_str();

        if let Some(result) = self.check_layer1_hoster_bypass(user_id, perm_key).await? {
            return Ok(result);
        }

        let Some(space_id) = space_id else {
            return Ok(PermissionResult::Denied("No space provided".to_string()));
        };

        if let Some(result) = self.check_layer2_membership(user_id, space_id).await? {
            return Ok(result);
        }

        let role_ids = self.repo.get_role_ids_for_user(space_id, user_id).await?;

        let role_allowed = self
            .check_layer3_role_permissions(&role_ids, perm_key)
            .await?;

        if let Some(channel_id) = channel_id {
            if let Some(result) = self
                .check_layer4_channel_override(&role_ids, channel_id, perm_key)
                .await?
            {
                return Ok(result);
            }

            if let Some(result) = self
                .check_layer5_feature_flags(channel_id, permission)
                .await?
            {
                return Ok(result);
            }
        }

        match role_allowed {
            Some(PermissionResult::Allowed) => Ok(PermissionResult::Allowed),
            Some(PermissionResult::Denied(reason)) => Ok(PermissionResult::Denied(reason)),
            None => {
                if role_ids.is_empty() {
                    Ok(PermissionResult::Denied("No roles assigned".to_string()))
                } else {
                    Ok(PermissionResult::Denied(
                        "Permission not granted by any role".to_string(),
                    ))
                }
            }
        }
    }

    async fn check_layer1_hoster_bypass(
        &self,
        user_id: Uuid,
        _permission: &str,
    ) -> Result<Option<PermissionResult>, AppError> {
        let host_user_id = self.repo.get_host_user_id().await?;

        if let Some(host_id) = host_user_id {
            if user_id == host_id {
                return Ok(Some(PermissionResult::Allowed));
            }
        }

        Ok(None)
    }

    async fn check_layer2_membership(
        &self,
        user_id: Uuid,
        space_id: Uuid,
    ) -> Result<Option<PermissionResult>, AppError> {
        let is_member = self.repo.is_space_member(space_id, user_id).await?;

        if !is_member {
            return Ok(Some(PermissionResult::Denied(
                "Not a member of this space".to_string(),
            )));
        }

        Ok(None)
    }

    async fn check_layer3_role_permissions(
        &self,
        role_ids: &[Uuid],
        permission: &str,
    ) -> Result<Option<PermissionResult>, AppError> {
        let role_permissions = self.repo.get_role_permissions(role_ids).await?;

        if role_permissions.is_empty() {
            return Ok(Some(PermissionResult::Denied(
                "No roles assigned".to_string(),
            )));
        }

        for rp in &role_permissions {
            if rp.permission_key == permission && rp.allowed {
                return Ok(Some(PermissionResult::Allowed));
            }
        }

        if role_permissions
            .iter()
            .any(|rp| rp.permission_key == permission)
        {
            Ok(Some(PermissionResult::Denied(
                "Permission explicitly denied by role".to_string(),
            )))
        } else {
            Ok(None)
        }
    }

    async fn check_layer4_channel_override(
        &self,
        role_ids: &[Uuid],
        channel_id: Uuid,
        permission: &str,
    ) -> Result<Option<PermissionResult>, AppError> {
        let overrides = self
            .repo
            .get_channel_overrides(channel_id, role_ids)
            .await?;

        for ov in overrides {
            if ov.permission_key == permission {
                if ov.denied {
                    return Ok(Some(PermissionResult::Denied(
                        "Permission denied by channel override".to_string(),
                    )));
                } else {
                    return Ok(Some(PermissionResult::Allowed));
                }
            }
        }

        Ok(None)
    }

    async fn check_layer5_feature_flags(
        &self,
        channel_id: Uuid,
        permission: PermissionKey,
    ) -> Result<Option<PermissionResult>, AppError> {
        let flags = self.repo.get_channel_feature_flags(channel_id).await?;

        let allowed = match permission {
            PermissionKey::SendMessages => flags.text_enabled,
            PermissionKey::SendFiles => flags.send_file_enabled,
            PermissionKey::JoinVoice | PermissionKey::StartVoice => flags.voice_group_enabled,
            PermissionKey::JoinVideo | PermissionKey::StartVideo | PermissionKey::ShareScreen => {
                flags.video_group_enabled
            }
            _ => true,
        };

        if !allowed {
            return Ok(Some(PermissionResult::Denied(format!(
                "Feature not enabled for this channel: {}",
                permission.as_str()
            ))));
        }

        Ok(None)
    }
}

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionResult {
    Allowed,
    Denied(String),
}

impl PermissionResult {
    pub fn is_allowed(&self) -> bool {
        matches!(self, PermissionResult::Allowed)
    }

    pub fn reason(&self) -> Option<&str> {
        match self {
            PermissionResult::Allowed => None,
            PermissionResult::Denied(reason) => Some(reason),
        }
    }
}
