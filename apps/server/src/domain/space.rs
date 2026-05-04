use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Space {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon_object_id: Option<Uuid>,
    pub created_by: Uuid,
    pub visibility: SpaceVisibility,
    pub settings: serde_json::Value,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SpaceVisibility {
    Public,
    Private,
}

impl std::fmt::Display for SpaceVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceVisibility::Public => write!(f, "public"),
            SpaceVisibility::Private => write!(f, "private"),
        }
    }
}

impl std::str::FromStr for SpaceVisibility {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(SpaceVisibility::Public),
            "private" => Ok(SpaceVisibility::Private),
            _ => Err(()),
        }
    }
}

impl Default for SpaceVisibility {
    fn default() -> Self {
        SpaceVisibility::Private
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateSpace {
    pub name: String,
    pub description: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpace {
    pub name: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
}