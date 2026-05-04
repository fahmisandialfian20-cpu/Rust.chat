use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub status: UserStatus,
    pub password_hash: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, ToSchema)]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deleted,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Pending => write!(f, "pending"),
            UserStatus::Active => write!(f, "active"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl std::str::FromStr for UserStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(UserStatus::Pending),
            "active" => Ok(UserStatus::Active),
            "suspended" => Ok(UserStatus::Suspended),
            "deleted" => Ok(UserStatus::Deleted),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub avatar_object_id: Option<Uuid>,
    pub bio: Option<String>,
    pub settings: serde_json::Value,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id: Uuid::nil(),
            display_name: None,
            avatar_object_id: None,
            bio: None,
            settings: serde_json::json!({}),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ThemePreferences {
    pub id: Uuid,
    pub user_id: Uuid,
    pub mode: String,
    pub accent: String,
    pub density: String,
    pub message_display: String,
    pub settings: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ClientInfo {
    pub client_type: String,
    pub platform: Option<String>,
    pub device_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct ClientDevice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub client_type: String,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub push_token: Option<String>,
    pub last_seen_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}