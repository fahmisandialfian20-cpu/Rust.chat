use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Channel {
    pub id: Uuid,
    pub space_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub kind: ChannelKind,
    pub visibility: ChannelVisibility,
    pub position: i32,
    pub topic: Option<String>,
    pub created_by: Uuid,
    pub archived_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Default)]
pub enum ChannelKind {
    #[default]
    Text,
    Voice,
    Video,
}

impl std::fmt::Display for ChannelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelKind::Text => write!(f, "text"),
            ChannelKind::Voice => write!(f, "voice"),
            ChannelKind::Video => write!(f, "video"),
        }
    }
}

impl std::str::FromStr for ChannelKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(ChannelKind::Text),
            "voice" => Ok(ChannelKind::Voice),
            "video" => Ok(ChannelKind::Video),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema, Default)]
pub enum ChannelVisibility {
    #[default]
    Public,
    Private,
}

impl std::fmt::Display for ChannelVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelVisibility::Public => write!(f, "public"),
            ChannelVisibility::Private => write!(f, "private"),
        }
    }
}

impl std::str::FromStr for ChannelVisibility {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(ChannelVisibility::Public),
            "private" => Ok(ChannelVisibility::Private),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ChannelFeatureFlags {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub text_enabled: bool,
    pub file_upload_enabled: bool,
    pub voice_group_enabled: bool,
    pub video_group_enabled: bool,
    pub threads_enabled: bool,
    pub reactions_enabled: bool,
}

impl Default for ChannelFeatureFlags {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            channel_id: Uuid::nil(),
            text_enabled: true,
            file_upload_enabled: true,
            voice_group_enabled: false,
            video_group_enabled: false,
            threads_enabled: true,
            reactions_enabled: true,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateChannel {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub kind: Option<String>,
    pub visibility: Option<String>,
    pub topic: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateChannel {
    pub name: Option<String>,
    pub topic: Option<String>,
    pub visibility: Option<String>,
    pub feature_flags: Option<ChannelFeatureFlagsUpdate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChannelFeatureFlagsUpdate {
    pub text_enabled: Option<bool>,
    pub file_upload_enabled: Option<bool>,
    pub voice_group_enabled: Option<bool>,
    pub video_group_enabled: Option<bool>,
    pub threads_enabled: Option<bool>,
    pub reactions_enabled: Option<bool>,
}
