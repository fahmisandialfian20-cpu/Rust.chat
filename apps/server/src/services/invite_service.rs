use crate::domain::invite::{CreateInvite, Invite};
use crate::error::AppError;
use crate::permissions::{PermissionKey, PermissionService};
use crate::repositories::channel_repository::ChannelRepository;
use crate::repositories::invite_repository::InviteRepository;
use crate::repositories::role_repository::RoleRepository;
use crate::repositories::space_repository::SpaceRepository;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone)]
pub struct InviteService {
    repository: Arc<InviteRepository>,
    space_repo: Arc<SpaceRepository>,
    channel_repo: Arc<ChannelRepository>,
    role_repo: Arc<RoleRepository>,
    permission_service: PermissionService,
}

impl InviteService {
    pub fn new(
        repository: Arc<InviteRepository>,
        space_repo: Arc<SpaceRepository>,
        channel_repo: Arc<ChannelRepository>,
        role_repo: Arc<RoleRepository>,
        permission_service: PermissionService,
    ) -> Self {
        Self {
            repository,
            space_repo,
            channel_repo,
            role_repo,
            permission_service,
        }
    }

    pub async fn create_invite(
        &self,
        user_id: Uuid,
        input: CreateInvite,
    ) -> Result<Invite, AppError> {
        if let Some(expires_at) = input.expires_at {
            if expires_at < OffsetDateTime::now_utc() {
                return Err(AppError::BadRequest(
                    "Expiration must be in the future".to_string(),
                ));
            }
        }

        let space_id =
            resolve_space_id(&self.channel_repo, input.space_id, input.channel_id).await?;

        self.permission_service
            .check(
                user_id,
                PermissionKey::ManageInvites,
                Some(space_id),
                input.channel_id,
            )
            .await?;

        self.repository
            .create(
                input.space_id,
                input.channel_id,
                user_id,
                input.max_uses,
                input.expires_at,
            )
            .await
    }

    pub async fn get_invite(&self, invite_id: Uuid) -> Result<Invite, AppError> {
        self.repository.find_by_id(invite_id).await
    }

    pub async fn get_invite_by_code(&self, code: &str) -> Result<Invite, AppError> {
        self.repository.find_by_code(code).await
    }

    pub async fn validate_invite(&self, code: &str) -> Result<Invite, AppError> {
        if !self.repository.is_valid(code).await? {
            return Err(AppError::BadRequest(
                "Invite is invalid or expired".to_string(),
            ));
        }

        self.repository.find_by_code(code).await
    }

    pub async fn consume_invite(&self, code: &str) -> Result<Invite, AppError> {
        let invite = self.validate_invite(code).await?;

        self.repository.increment_used_count(invite.id).await?;

        Ok(invite)
    }

    pub async fn accept_invite(&self, code: &str, user_id: Uuid) -> Result<String, AppError> {
        let invite = self.repository.find_by_code(code).await?;
        let invite = self.repository.try_consume(invite.id).await?;

        if let Some(space_id) = invite.space_id {
            let membership = self.space_repo.add_member(space_id, user_id, None).await?;

            let roles = self.role_repo.find_by_space(space_id).await?;
            if let Some(everyone) = roles.iter().find(|r| r.is_default) {
                self.role_repo
                    .assign_role_to_member(membership.id, everyone.id)
                    .await?;
            }

            Ok(space_id.to_string())
        } else if let Some(channel_id) = invite.channel_id {
            self.channel_repo.add_member(channel_id, user_id).await?;

            Ok(channel_id.to_string())
        } else {
            Err(AppError::BadRequest("Invite has no target".to_string()))
        }
    }

    pub async fn list_space_invites(
        &self,
        space_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Invite>, AppError> {
        self.repository.find_by_space(space_id, limit, offset).await
    }

    pub async fn delete_invite(&self, invite_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(invite_id).await
    }
}

async fn resolve_space_id(
    channel_repo: &ChannelRepository,
    space_id: Option<Uuid>,
    channel_id: Option<Uuid>,
) -> Result<Uuid, AppError> {
    if let Some(sid) = space_id {
        return Ok(sid);
    }
    if let Some(cid) = channel_id {
        let channel = channel_repo.find_by_id(cid).await?;
        return Ok(channel.space_id);
    }
    Err(AppError::BadRequest(
        "Invite must target a space or channel".to_string(),
    ))
}
