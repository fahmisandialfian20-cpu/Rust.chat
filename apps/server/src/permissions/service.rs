use sqlx::PgPool;
use uuid::Uuid;
use crate::error::AppError;
use super::keys::PermissionKey;
use super::resolver::{PermissionResolver, PermissionResult};

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
        let result = self.resolver.check(user_id, permission, space_id, channel_id).await?;

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
        let result = self.resolver.check(user_id, permission, space_id, channel_id).await?;
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
            let result = self.resolver.check(user_id, *permission, space_id, channel_id).await?;
            if result.is_allowed() {
                return Ok(true);
            }
        }
        Ok(false)
    }
}