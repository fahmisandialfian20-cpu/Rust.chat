use crate::domain::message::{CreateMessage, Message, UpdateMessage};
use crate::error::AppError;
use crate::repositories::message_repository::MessageRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct MessageService {
    repository: Arc<MessageRepository>,
}

impl MessageService {
    pub fn new(repository: Arc<MessageRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_message(
        &self,
        channel_id: Uuid,
        author_user_id: Uuid,
        input: CreateMessage,
    ) -> Result<Message, AppError> {
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

    pub async fn get_message(&self, message_id: Uuid) -> Result<Message, AppError> {
        self.repository.find_by_id(message_id).await
    }

    pub async fn list_channel_messages(
        &self,
        channel_id: Uuid,
        limit: i64,
        before: Option<Uuid>,
    ) -> Result<Vec<Message>, AppError> {
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

        if existing.author_user_id != user_id {
            return Err(AppError::Forbidden(
                "You can only edit your own messages".to_string(),
            ));
        }

        if existing.deleted_at.is_some() {
            return Err(AppError::NotFound("Message not found".to_string()));
        }

        self.repository.update(message_id, input.content).await
    }

    pub async fn delete_message(&self, message_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let existing = self.repository.find_by_id(message_id).await?;

        if existing.author_user_id != user_id {
            return Err(AppError::Forbidden(
                "You can only delete your own messages".to_string(),
            ));
        }

        self.repository.soft_delete(message_id).await
    }
}
