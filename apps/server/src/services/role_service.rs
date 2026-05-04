use std::sync::Arc;
use uuid::Uuid;

use crate::domain::role::{RoleWithPermissions, CreateRoleRequest, UpdateRoleRequest};
use crate::repositories::role_repository::RoleRepository;
use crate::permissions::PermissionService;
use crate::permissions::PermissionKey;
use crate::error::AppError;

#[derive(Clone)]
pub struct RoleService {
    repository: Arc<RoleRepository>,
    permission_service: PermissionService,
}

impl RoleService {
    pub fn new(
        repository: Arc<RoleRepository>,
        permission_service: PermissionService,
    ) -> Self {
        Self { repository, permission_service }
    }

    pub async fn create_role(
        &self,
        space_id: Uuid,
        actor: Uuid,
        input: CreateRoleRequest,
    ) -> Result<RoleWithPermissions, AppError> {
        self.permission_service.check(actor, PermissionKey::ManageRoles, Some(space_id), None).await?;

        let role = self.repository.create(space_id, &input.name, false).await?;

        if !input.permission_keys.is_empty() {
            self.repository.set_permissions(role.id, &input.permission_keys, true).await?;
        }

        let keys = self.repository.get_permission_keys(role.id).await?;

        Ok(RoleWithPermissions { role, permission_keys: keys })
    }

    pub async fn list_roles(
        &self,
        space_id: Uuid,
        actor: Uuid,
    ) -> Result<Vec<RoleWithPermissions>, AppError> {
        self.permission_service.check(actor, PermissionKey::ViewSpace, Some(space_id), None).await?;

        let roles = self.repository.find_by_space(space_id).await?;
        let mut result = Vec::new();

        for role in roles {
            let keys = self.repository.get_permission_keys(role.id).await?;
            result.push(RoleWithPermissions { role, permission_keys: keys });
        }

        Ok(result)
    }

    pub async fn get_role(
        &self,
        role_id: Uuid,
        actor: Uuid,
    ) -> Result<RoleWithPermissions, AppError> {
        let role = self.repository.find_by_id(role_id).await?;

        self.permission_service.check(actor, PermissionKey::ViewSpace, Some(role.space_id), None).await?;

        let keys = self.repository.get_permission_keys(role.id).await?;

        Ok(RoleWithPermissions { role, permission_keys: keys })
    }

    pub async fn update_role(
        &self,
        role_id: Uuid,
        actor: Uuid,
        input: UpdateRoleRequest,
    ) -> Result<RoleWithPermissions, AppError> {
        let role = self.repository.find_by_id(role_id).await?;

        self.permission_service.check(actor, PermissionKey::ManageRoles, Some(role.space_id), None).await?;

        let name = input.name.unwrap_or(role.name);

        let updated = self.repository.update(role_id, &name).await?;

        if let Some(keys) = input.permission_keys {
            self.repository.set_permissions(role_id, &keys, true).await?;
        }

        let permission_keys = self.repository.get_permission_keys(role_id).await?;

        Ok(RoleWithPermissions { role: updated, permission_keys })
    }

    pub async fn delete_role(
        &self,
        role_id: Uuid,
        actor: Uuid,
    ) -> Result<(), AppError> {
        let role = self.repository.find_by_id(role_id).await?;

        if role.is_default {
            return Err(AppError::BadRequest("Cannot delete the @everyone default role".to_string()));
        }

        self.permission_service.check(actor, PermissionKey::ManageRoles, Some(role.space_id), None).await?;

        self.repository.delete(role_id).await
    }

    pub async fn assign_role(
        &self,
        space_id: Uuid,
        member_user_id: Uuid,
        role_id: Uuid,
        actor: Uuid,
    ) -> Result<(), AppError> {
        self.permission_service.check(actor, PermissionKey::ManageRoles, Some(space_id), None).await?;

        let membership_id = self.repository.find_membership_id(space_id, member_user_id).await?;

        self.repository.assign_role_to_member(membership_id, role_id).await
    }

    pub async fn remove_role(
        &self,
        space_id: Uuid,
        member_user_id: Uuid,
        role_id: Uuid,
        actor: Uuid,
    ) -> Result<(), AppError> {
        self.permission_service.check(actor, PermissionKey::ManageRoles, Some(space_id), None).await?;

        let membership_id = self.repository.find_membership_id(space_id, member_user_id).await?;

        self.repository.remove_role_from_member(membership_id, role_id).await
    }
}
