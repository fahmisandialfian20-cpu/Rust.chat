use crate::domain::message::{CreateMessage, Message, UpdateMessage};
use crate::error::AppError;
use crate::permissions::{PermissionKey, PermissionService};
use crate::repositories::channel_repository::ChannelRepository;
use crate::repositories::message_repository::MessageRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MessageService {
    repository: Arc<MessageRepository>,
    permission_service: PermissionService,
    channel_repository: Arc<ChannelRepository>,
}

impl MessageService {
    pub fn new(
        repository: Arc<MessageRepository>,
        permission_service: PermissionService,
        channel_repository: Arc<ChannelRepository>,
    ) -> Self {
        Self {
            repository,
            permission_service,
            channel_repository,
        }
    }

    pub async fn create_message(
        &self,
        channel_id: Uuid,
        author_user_id: Uuid,
        input: CreateMessage,
    ) -> Result<Message, AppError> {
        let channel = self.channel_repository.find_by_id(channel_id).await?;

        self.permission_service
            .check(
                author_user_id,
                PermissionKey::SendMessages,
                Some(channel.space_id),
                Some(channel_id),
            )
            .await?;

        let kind = input.kind.unwrap_or_else(|| "text".to_string());

        self.repository
            .create(
                channel_id,
                author_user_id,
                input.content,
                kind,
                input.reply_to_message_id,
            )
            .await
    }

    pub async fn get_message(&self, message_id: Uuid, user_id: Uuid) -> Result<Message, AppError> {
        let message = self.repository.find_by_id(message_id).await?;
        let channel = self
            .channel_repository
            .find_by_id(message.channel_id)
            .await?;

        self.permission_service
            .check(
                user_id,
                PermissionKey::ReadMessages,
                Some(channel.space_id),
                Some(message.channel_id),
            )
            .await?;

        Ok(message)
    }

    pub async fn list_channel_messages(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
        limit: i64,
        before: Option<Uuid>,
    ) -> Result<Vec<Message>, AppError> {
        let channel = self.channel_repository.find_by_id(channel_id).await?;

        self.permission_service
            .check(
                user_id,
                PermissionKey::ReadMessages,
                Some(channel.space_id),
                Some(channel_id),
            )
            .await?;

        self.repository
            .find_by_channel(channel_id, limit, before)
            .await
    }

    pub async fn update_message(
        &self,
        message_id: Uuid,
        user_id: Uuid,
        input: UpdateMessage,
    ) -> Result<Message, AppError> {
        let existing = self.repository.find_by_id(message_id).await?;
        let channel = self
            .channel_repository
            .find_by_id(existing.channel_id)
            .await?;

        if existing.author_user_id == user_id {
            self.permission_service
                .check(
                    user_id,
                    PermissionKey::EditOwnMessage,
                    Some(channel.space_id),
                    Some(existing.channel_id),
                )
                .await?;
        } else {
            self.permission_service
                .check(
                    user_id,
                    PermissionKey::EditAnyMessage,
                    Some(channel.space_id),
                    Some(existing.channel_id),
                )
                .await?;
        }

        if existing.deleted_at.is_some() {
            return Err(AppError::NotFound("Message not found".to_string()));
        }

        self.repository.update(message_id, input.content).await
    }

    pub async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let existing = self.repository.find_by_id(message_id).await?;
        let channel = self
            .channel_repository
            .find_by_id(existing.channel_id)
            .await?;

        if existing.author_user_id == user_id {
            self.permission_service
                .check(
                    user_id,
                    PermissionKey::DeleteOwnMessage,
                    Some(channel.space_id),
                    Some(existing.channel_id),
                )
                .await?;
        } else {
            self.permission_service
                .check(
                    user_id,
                    PermissionKey::DeleteAnyMessage,
                    Some(channel.space_id),
                    Some(existing.channel_id),
                )
                .await?;
        }

        self.repository.soft_delete(message_id).await
    }
}
