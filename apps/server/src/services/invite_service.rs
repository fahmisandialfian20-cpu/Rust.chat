use uuid::Uuid;
use time::OffsetDateTime;
use crate::domain::invite::{Invite, CreateInvite};
use crate::repositories::invite_repository::InviteRepository;
use crate::error::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct InviteService {
    repository: Arc<InviteRepository>,
}

impl InviteService {
    pub fn new(repository: Arc<InviteRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_invite(
        &self,
        user_id: Uuid,
        input: CreateInvite,
    ) -> Result<Invite, AppError> {
        if let Some(expires_at) = input.expires_at {
            if expires_at < OffsetDateTime::now_utc() {
                return Err(AppError::BadRequest("Expiration must be in the future".to_string()));
            }
        }

        self.repository.create(
            input.space_id,
            input.channel_id,
            user_id,
            input.max_uses,
            input.expires_at,
        ).await
    }

    pub async fn get_invite(&self, invite_id: Uuid) -> Result<Invite, AppError> {
        self.repository.find_by_id(invite_id).await
    }

    pub async fn get_invite_by_code(&self, code: &str) -> Result<Invite, AppError> {
        self.repository.find_by_code(code).await
    }

    pub async fn validate_invite(&self, code: &str) -> Result<Invite, AppError> {
        if !self.repository.is_valid(code).await? {
            return Err(AppError::BadRequest("Invite is invalid or expired".to_string()));
        }

        self.repository.find_by_code(code).await
    }

    pub async fn consume_invite(&self, code: &str) -> Result<Invite, AppError> {
        let invite = self.validate_invite(code).await?;

        self.repository.increment_used_count(invite.id).await?;

        Ok(invite)
    }

    pub async fn list_space_invites(&self, space_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Invite>, AppError> {
        self.repository.find_by_space(space_id, limit, offset).await
    }

    pub async fn delete_invite(&self, invite_id: Uuid) -> Result<(), AppError> {
        self.repository.delete(invite_id).await
    }
}