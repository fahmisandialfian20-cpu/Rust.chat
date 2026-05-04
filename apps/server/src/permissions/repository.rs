use crate::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PermissionRepository {
    pub pool: PgPool,
}

impl PermissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_host_user_id(&self) -> Result<Option<Uuid>, AppError> {
        let result = sqlx::query_scalar::<_, Option<Uuid>>(
            "SELECT owner_user_id FROM instance_settings WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.flatten())
    }

    pub async fn is_space_member(&self, space_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM space_memberships WHERE space_id = $1 AND user_id = $2)",
        )
        .bind(space_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    pub async fn get_role_ids_for_user(
        &self,
        space_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<Uuid>, AppError> {
        let rows = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT mr.role_id FROM member_roles mr
            JOIN space_memberships sm ON mr.membership_id = sm.id
            WHERE sm.space_id = $1 AND sm.user_id = $2
            "#,
        )
        .bind(space_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows)
    }

    pub async fn get_role_permissions(
        &self,
        role_ids: &[Uuid],
    ) -> Result<Vec<RolePermission>, AppError> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        let rows = sqlx::query_as::<_, SqlxRolePermission>(
            r#"
            SELECT role_id, permission_key, allowed
            FROM role_permissions
            WHERE role_id = ANY($1)
            "#,
        )
        .bind(role_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_channel_overrides(
        &self,
        channel_id: Uuid,
        role_ids: &[Uuid],
    ) -> Result<Vec<ChannelOverride>, AppError> {
        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        let rows = sqlx::query_as::<_, SqlxChannelOverride>(
            r#"
            SELECT channel_id, role_id, permission_key, denied
            FROM channel_permission_overrides
            WHERE channel_id = $1 AND role_id = ANY($2)
            "#,
        )
        .bind(channel_id)
        .bind(role_ids)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_channel_feature_flags(
        &self,
        channel_id: Uuid,
    ) -> Result<ChannelFeatureFlags, AppError> {
        let row = sqlx::query_as::<_, SqlxChannelFeatureFlags>(
            r#"
            SELECT channel_id, text_enabled, send_file_enabled, voice_group_enabled, video_group_enabled
            FROM channel_feature_flags
            WHERE channel_id = $1
            "#,
        )
        .bind(channel_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.map(|r| r.into()).unwrap_or_default())
    }

    pub async fn get_channel(&self, channel_id: Uuid) -> Result<Channel, AppError> {
        let row = sqlx::query_as::<_, SqlxChannel>(
            r#"
            SELECT id, space_id, name, slug, kind, visibility
            FROM channels WHERE id = $1
            "#,
        )
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Channel not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }
}

#[derive(Debug, Clone)]
pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_key: String,
    pub allowed: bool,
}

#[derive(Debug, Clone)]
pub struct ChannelOverride {
    pub channel_id: Uuid,
    pub role_id: Uuid,
    pub permission_key: String,
    pub denied: bool,
}

#[derive(Debug, Clone)]
pub struct ChannelFeatureFlags {
    pub text_enabled: bool,
    pub send_file_enabled: bool,
    pub voice_group_enabled: bool,
    pub video_group_enabled: bool,
}

impl Default for ChannelFeatureFlags {
    fn default() -> Self {
        Self {
            text_enabled: true,
            send_file_enabled: true,
            voice_group_enabled: false,
            video_group_enabled: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub id: Uuid,
    pub space_id: Uuid,
    pub name: String,
    pub slug: String,
    pub kind: String,
    pub visibility: String,
}

#[derive(sqlx::FromRow)]
struct SqlxRolePermission {
    role_id: Uuid,
    permission_key: String,
    allowed: bool,
}

impl From<SqlxRolePermission> for RolePermission {
    fn from(row: SqlxRolePermission) -> Self {
        RolePermission {
            role_id: row.role_id,
            permission_key: row.permission_key,
            allowed: row.allowed,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SqlxChannelOverride {
    channel_id: Uuid,
    role_id: Uuid,
    permission_key: String,
    denied: bool,
}

impl From<SqlxChannelOverride> for ChannelOverride {
    fn from(row: SqlxChannelOverride) -> Self {
        ChannelOverride {
            channel_id: row.channel_id,
            role_id: row.role_id,
            permission_key: row.permission_key,
            denied: row.denied,
        }
    }
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct SqlxChannelFeatureFlags {
    channel_id: Uuid,
    text_enabled: bool,
    send_file_enabled: bool,
    voice_group_enabled: bool,
    video_group_enabled: bool,
}

impl From<SqlxChannelFeatureFlags> for ChannelFeatureFlags {
    fn from(row: SqlxChannelFeatureFlags) -> Self {
        ChannelFeatureFlags {
            text_enabled: row.text_enabled,
            send_file_enabled: row.send_file_enabled,
            voice_group_enabled: row.voice_group_enabled,
            video_group_enabled: row.video_group_enabled,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SqlxChannel {
    id: Uuid,
    space_id: Uuid,
    name: String,
    slug: String,
    kind: String,
    visibility: String,
}

impl From<SqlxChannel> for Channel {
    fn from(row: SqlxChannel) -> Self {
        Channel {
            id: row.id,
            space_id: row.space_id,
            name: row.name,
            slug: row.slug,
            kind: row.kind,
            visibility: row.visibility,
        }
    }
}
