use crate::domain::channel::{
    Channel, ChannelFeatureFlags, ChannelFeatureFlagsUpdate, CreateChannel, UpdateChannel,
};
use crate::error::AppError;
use crate::realtime::events::WsEvent;
use crate::realtime::RealtimeHub;
use crate::repositories::channel_repository::ChannelRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChannelService {
    repository: Arc<ChannelRepository>,
    hub: Arc<RealtimeHub>,
}

impl ChannelService {
    pub fn new(repository: Arc<ChannelRepository>, hub: Arc<RealtimeHub>) -> Self {
        Self { repository, hub }
    }

    pub async fn create_channel(
        &self,
        space_id: Uuid,
        user_id: Uuid,
        input: CreateChannel,
    ) -> Result<Channel, AppError> {
        let slug = Self::generate_slug(&input.name);

        if self.repository.slug_exists(space_id, &slug).await? {
            return Err(AppError::Conflict(
                "Channel with similar name already exists".to_string(),
            ));
        }

        let kind = input
            .kind
            .as_ref()
            .and_then(|k| k.parse().ok())
            .unwrap_or_default();

        let visibility = input
            .visibility
            .as_ref()
            .and_then(|v| v.parse().ok())
            .unwrap_or_default();

        let position = self
            .repository
            .get_next_position(space_id, input.parent_id)
            .await?;

        let channel = self
            .repository
            .create(
                space_id,
                input.name,
                slug,
                input.parent_id,
                kind,
                visibility,
                input.topic,
                position,
                user_id,
            )
            .await?;

        if let Ok(json) = WsEvent::ChannelCreated(channel.clone()).to_json() {
            self.hub.publish_to_channel(channel.id, json).await;
        }

        Ok(channel)
    }

    pub async fn get_channel(&self, channel_id: Uuid) -> Result<Channel, AppError> {
        self.repository.find_by_id(channel_id).await
    }

    pub async fn get_channel_by_slug(
        &self,
        space_id: Uuid,
        slug: &str,
    ) -> Result<Channel, AppError> {
        self.repository.find_by_slug(space_id, slug).await
    }

    pub async fn list_space_channels(
        &self,
        space_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Channel>, AppError> {
        self.repository.find_by_space(space_id, limit, offset).await
    }

    pub async fn list_visible_channels(
        &self,
        space_id: Uuid,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Channel>, AppError> {
        self.repository
            .find_by_space_visible_to_user(space_id, user_id, limit, offset)
            .await
    }

    pub async fn update_channel(
        &self,
        channel_id: Uuid,
        input: UpdateChannel,
    ) -> Result<Channel, AppError> {
        let _ = self.repository.find_by_id(channel_id).await?;
        let visibility_changed = input.visibility.is_some();
        let updated = self
            .repository
            .update(channel_id, input.name, input.topic, input.visibility)
            .await?;
        if let Ok(json) = WsEvent::ChannelUpdated(updated.clone()).to_json() {
            self.hub.publish_to_channel(updated.id, json).await;
        }
        if visibility_changed {
            if let Ok(json) = WsEvent::ChannelVisibilityChanged(updated.id).to_json() {
                self.hub.publish_to_channel(updated.id, json).await;
            }
        }
        Ok(updated)
    }

    pub async fn archive_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
        let channel = self.repository.find_by_id(channel_id).await?;
        self.repository.archive(channel_id).await?;
        if let Ok(json) = WsEvent::ChannelUpdated(channel).to_json() {
            self.hub.publish_to_channel(channel_id, json).await;
        }
        Ok(())
    }

    pub async fn delete_channel(&self, channel_id: Uuid) -> Result<(), AppError> {
        self.repository.find_by_id(channel_id).await?;
        self.repository.delete(channel_id).await?;
        if let Ok(json) = WsEvent::ChannelDeleted(channel_id).to_json() {
            self.hub.publish_to_channel(channel_id, json).await;
        }
        Ok(())
    }

    pub async fn get_feature_flags(
        &self,
        channel_id: Uuid,
    ) -> Result<ChannelFeatureFlags, AppError> {
        self.repository.get_feature_flags(channel_id).await
    }

    pub async fn update_feature_flags(
        &self,
        channel_id: Uuid,
        input: ChannelFeatureFlagsUpdate,
    ) -> Result<ChannelFeatureFlags, AppError> {
        self.repository
            .update_feature_flags(
                channel_id,
                input.text_enabled,
                input.file_upload_enabled,
                input.voice_group_enabled,
                input.video_group_enabled,
                input.threads_enabled,
                input.reactions_enabled,
            )
            .await
    }

    pub async fn add_member(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.add_member(channel_id, user_id).await
    }

    pub async fn remove_member(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repository.remove_member(channel_id, user_id).await
    }

    fn generate_slug(name: &str) -> String {
        let slug = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>();

        let slug = slug.trim_matches('-').to_string();

        if slug.is_empty() {
            format!("channel-{}", &Uuid::new_v4().to_string()[..8])
        } else {
            slug
        }
    }
}
