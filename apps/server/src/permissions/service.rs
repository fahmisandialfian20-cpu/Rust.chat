use super::keys::PermissionKey;
use super::resolver::{PermissionResolver, PermissionResult};
use crate::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PermissionService {
    resolver: PermissionResolver,
}

impl PermissionService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            resolver: PermissionResolver::new(pool),
        }
    }

    pub async fn check(
        &self,
        user_id: Uuid,
        permission: PermissionKey,
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
    ) -> Result<(), AppError> {
        let result = self
            .resolver
            .check(user_id, permission, space_id, channel_id)
            .await?;

        match result {
            PermissionResult::Allowed => Ok(()),
            PermissionResult::Denied(reason) => Err(AppError::Forbidden(reason)),
        }
    }

    pub async fn check_optional(
        &self,
        user_id: Uuid,
        permission: PermissionKey,
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        let result = self
            .resolver
            .check(user_id, permission, space_id, channel_id)
            .await?;
        Ok(result.is_allowed())
    }

    pub async fn is_space_member(&self, user_id: Uuid, space_id: Uuid) -> Result<bool, AppError> {
        self.resolver.is_space_member(user_id, space_id).await
    }

    pub async fn has_any_permission(
        &self,
        user_id: Uuid,
        permissions: &[PermissionKey],
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        for permission in permissions {
            let result = self
                .resolver
                .check(user_id, *permission, space_id, channel_id)
                .await?;
            if result.is_allowed() {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn list_user_permissions(
        &self,
        user_id: Uuid,
        space_id: Uuid,
    ) -> Result<Vec<String>, AppError> {
        let all_keys = [
            PermissionKey::ManageInstance,
            PermissionKey::ManageSpaces,
            PermissionKey::ManageRoles,
            PermissionKey::ManageMembers,
            PermissionKey::ManageChannels,
            PermissionKey::ManageInvites,
            PermissionKey::ViewAuditLog,
            PermissionKey::ViewSpace,
            PermissionKey::ViewChannel,
            PermissionKey::ReadMessages,
            PermissionKey::SendMessages,
            PermissionKey::EditOwnMessage,
            PermissionKey::DeleteOwnMessage,
            PermissionKey::EditAnyMessage,
            PermissionKey::DeleteAnyMessage,
            PermissionKey::PinMessages,
            PermissionKey::MentionEveryone,
            PermissionKey::SendFiles,
            PermissionKey::CreateThreads,
            PermissionKey::ManageThreads,
            PermissionKey::AddReactions,
            PermissionKey::JoinVoice,
            PermissionKey::StartVoice,
            PermissionKey::JoinVideo,
            PermissionKey::StartVideo,
            PermissionKey::ShareScreen,
            PermissionKey::KickMembers,
            PermissionKey::BanMembers,
            PermissionKey::MuteMembers,
            PermissionKey::ManageModeration,
            PermissionKey::CustomizeOwnProfile,
            PermissionKey::CustomizeSpace,
            PermissionKey::UseWebhooks,
        ];

        let mut allowed = Vec::new();
        for key in all_keys {
            if self
                .resolver
                .check(user_id, key, Some(space_id), None)
                .await?
                .is_allowed()
            {
                allowed.push(key.as_str().to_string());
            }
        }
        Ok(allowed)
    }
}
