use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::channel::{Channel, ChannelFeatureFlags, ChannelKind, ChannelVisibility};
use crate::error::AppError;

pub struct ChannelRepository {
    pool: PgPool,
}

impl ChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        space_id: Uuid,
        name: String,
        slug: String,
        parent_id: Option<Uuid>,
        kind: ChannelKind,
        visibility: ChannelVisibility,
        topic: Option<String>,
        position: i32,
        created_by: Uuid,
    ) -> Result<Channel, AppError> {
        let row = sqlx::query_as::<_, SqlxChannel>(
            r#"
            INSERT INTO channels (id, space_id, parent_id, name, slug, kind, visibility, position, topic, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
            RETURNING id, space_id, parent_id, name, slug, kind, visibility, position, topic, created_by, archived_at, created_at, updated_at
            "#
        )
        .bind(Uuid::now_v7())
        .bind(space_id)
        .bind(parent_id)
        .bind(&name)
        .bind(&slug)
        .bind(kind.to_string())
        .bind(visibility.to_string())
        .bind(position)
        .bind(&topic)
        .bind(created_by)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let channel: Channel = row.into();

        sqlx::query(
            r#"
            INSERT INTO channel_feature_flags (id, channel_id, text_enabled, send_file_enabled, voice_group_enabled, video_group_enabled, threads_enabled, reactions_enabled)
            VALUES ($1, $2, true, true, false, false, true, true)
            "#
        )
        .bind(Uuid::now_v7())
        .bind(channel.id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(channel)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Channel, AppError> {
        let row = sqlx::query_as::<_, SqlxChannel>(
            r#"
            SELECT id, space_id, parent_id, name, slug, kind, visibility, position, topic, created_by, archived_at, created_at, updated_at
            FROM channels WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Channel not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_slug(&self, space_id: Uuid, slug: &str) -> Result<Channel, AppError> {
        let row = sqlx::query_as::<_, SqlxChannel>(
            r#"
            SELECT id, space_id, parent_id, name, slug, kind, visibility, position, topic, created_by, archived_at, created_at, updated_at
            FROM channels WHERE space_id = $1 AND slug = $2
            "#
        )
        .bind(space_id)
        .bind(slug)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Channel not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_space(
        &self,
        space_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Channel>, AppError> {
        let rows = sqlx::query_as::<_, SqlxChannel>(
            r#"
            SELECT id, space_id, parent_id, name, slug, kind, visibility, position, topic, created_by, archived_at, created_at, updated_at
            FROM channels
            WHERE space_id = $1 AND archived_at IS NULL
            ORDER BY position ASC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(space_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn find_by_space_visible_to_user(
        &self,
        space_id: Uuid,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Channel>, AppError> {
        let rows = sqlx::query_as::<_, SqlxChannel>(
            r#"
            SELECT c.id, c.space_id, c.parent_id, c.name, c.slug, c.kind, c.visibility, c.position, c.topic, c.created_by, c.archived_at, c.created_at, c.updated_at
            FROM channels c
            LEFT JOIN channel_memberships cm ON c.id = cm.channel_id AND cm.user_id = $2
            WHERE c.space_id = $1 
              AND c.visibility = 'public'
              OR (c.visibility = 'private' AND cm.user_id IS NOT NULL)
              AND c.archived_at IS NULL
            ORDER BY c.position ASC
            LIMIT $3 OFFSET $4
            "#
        )
        .bind(space_id)
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        topic: Option<String>,
        visibility: Option<String>,
    ) -> Result<Channel, AppError> {
        let mut updates = Vec::new();
        let mut bind_idx = 1;

        if name.is_some() {
            updates.push(format!("name = ${}", bind_idx));
            bind_idx += 1;
        }
        if topic.is_some() {
            updates.push(format!("topic = ${}", bind_idx));
            bind_idx += 1;
        }
        if visibility.is_some() {
            updates.push(format!("visibility = ${}", bind_idx));
            bind_idx += 1;
        }

        if updates.is_empty() {
            return self.find_by_id(id).await;
        }

        updates.push(format!("updated_at = ${}", bind_idx));
        bind_idx += 1;

        let query = format!(
            "UPDATE channels SET {} WHERE id = ${} RETURNING id, space_id, parent_id, name, slug, kind, visibility, position, topic, created_by, archived_at, created_at, updated_at",
            updates.join(", "),
            bind_idx
        );

        let mut query_builder = sqlx::query_as::<_, SqlxChannel>(&query);

        if let Some(ref n) = name {
            query_builder = query_builder.bind(n);
        }
        if let Some(ref t) = topic {
            query_builder = query_builder.bind(t);
        }
        if let Some(ref v) = visibility {
            query_builder = query_builder.bind(v);
        }
        query_builder = query_builder.bind(OffsetDateTime::now_utc());
        query_builder = query_builder.bind(id);

        let row = query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("Channel not found".to_string()),
                _ => AppError::InternalServerError(e.to_string()),
            })?;

        Ok(row.into())
    }

    pub async fn archive(&self, id: Uuid) -> Result<(), AppError> {
        let _row = sqlx::query("UPDATE channels SET archived_at = $1 WHERE id = $2")
            .bind(OffsetDateTime::now_utc())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("Channel not found".to_string()),
                _ => AppError::InternalServerError(e.to_string()),
            })?;

        Ok(())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM channels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Channel not found".to_string()));
        }

        Ok(())
    }

    pub async fn slug_exists(&self, space_id: Uuid, slug: &str) -> Result<bool, AppError> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM channels WHERE space_id = $1 AND slug = $2)",
        )
        .bind(space_id)
        .bind(slug)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    pub async fn get_next_position(
        &self,
        space_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<i32, AppError> {
        let result = sqlx::query_scalar::<_, Option<i32>>(
            r#"
            SELECT MAX(position) FROM channels WHERE space_id = $1 AND parent_id IS NOT DISTINCT FROM $2
            "#
        )
        .bind(space_id)
        .bind(parent_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.unwrap_or(0) + 1)
    }

    pub async fn add_member(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO channel_memberships (id, channel_id, user_id, created_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (channel_id, user_id) DO NOTHING
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(channel_id)
        .bind(user_id)
        .bind(OffsetDateTime::now_utc())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn remove_member(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result =
            sqlx::query("DELETE FROM channel_memberships WHERE channel_id = $1 AND user_id = $2")
                .bind(channel_id)
                .bind(user_id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Membership not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_feature_flags(
        &self,
        channel_id: Uuid,
    ) -> Result<ChannelFeatureFlags, AppError> {
        let row = sqlx::query_as::<_, SqlxChannelFeatureFlags>(
            r#"
            SELECT id, channel_id, text_enabled, file_upload_enabled, voice_group_enabled, video_group_enabled, threads_enabled, reactions_enabled
            FROM channel_feature_flags WHERE channel_id = $1
            "#
        )
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn update_feature_flags(
        &self,
        channel_id: Uuid,
        text_enabled: Option<bool>,
        file_upload_enabled: Option<bool>,
        voice_group_enabled: Option<bool>,
        video_group_enabled: Option<bool>,
        threads_enabled: Option<bool>,
        reactions_enabled: Option<bool>,
    ) -> Result<ChannelFeatureFlags, AppError> {
        let mut updates = Vec::new();
        let mut bind_idx = 1;

        if text_enabled.is_some() {
            updates.push(format!("text_enabled = ${}", bind_idx));
            bind_idx += 1;
        }
        if file_upload_enabled.is_some() {
            updates.push(format!("file_upload_enabled = ${}", bind_idx));
            bind_idx += 1;
        }
        if voice_group_enabled.is_some() {
            updates.push(format!("voice_group_enabled = ${}", bind_idx));
            bind_idx += 1;
        }
        if video_group_enabled.is_some() {
            updates.push(format!("video_group_enabled = ${}", bind_idx));
            bind_idx += 1;
        }
        if threads_enabled.is_some() {
            updates.push(format!("threads_enabled = ${}", bind_idx));
            bind_idx += 1;
        }
        if reactions_enabled.is_some() {
            updates.push(format!("reactions_enabled = ${}", bind_idx));
            bind_idx += 1;
        }

        if updates.is_empty() {
            return self.get_feature_flags(channel_id).await;
        }

        let query = format!(
            "UPDATE channel_feature_flags SET {} WHERE channel_id = ${} RETURNING id, channel_id, text_enabled, file_upload_enabled, voice_group_enabled, video_group_enabled, threads_enabled, reactions_enabled",
            updates.join(", "),
            bind_idx
        );

        let mut query_builder = sqlx::query_as::<_, SqlxChannelFeatureFlags>(&query);

        if let Some(v) = text_enabled {
            query_builder = query_builder.bind(v);
        }
        if let Some(v) = file_upload_enabled {
            query_builder = query_builder.bind(v);
        }
        if let Some(v) = voice_group_enabled {
            query_builder = query_builder.bind(v);
        }
        if let Some(v) = video_group_enabled {
            query_builder = query_builder.bind(v);
        }
        if let Some(v) = threads_enabled {
            query_builder = query_builder.bind(v);
        }
        if let Some(v) = reactions_enabled {
            query_builder = query_builder.bind(v);
        }
        query_builder = query_builder.bind(channel_id);

        let row = query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct SqlxChannel {
    id: Uuid,
    space_id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    slug: String,
    kind: String,
    visibility: String,
    position: i32,
    topic: Option<String>,
    created_by: Uuid,
    archived_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<SqlxChannel> for Channel {
    fn from(row: SqlxChannel) -> Self {
        Channel {
            id: row.id,
            space_id: row.space_id,
            parent_id: row.parent_id,
            name: row.name,
            slug: row.slug,
            kind: row.kind.parse().unwrap_or_default(),
            visibility: row.visibility.parse().unwrap_or_default(),
            position: row.position,
            topic: row.topic,
            created_by: row.created_by,
            archived_at: row.archived_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SqlxChannelFeatureFlags {
    id: Uuid,
    channel_id: Uuid,
    text_enabled: bool,
    file_upload_enabled: bool,
    voice_group_enabled: bool,
    video_group_enabled: bool,
    threads_enabled: bool,
    reactions_enabled: bool,
}

impl From<SqlxChannelFeatureFlags> for ChannelFeatureFlags {
    fn from(row: SqlxChannelFeatureFlags) -> Self {
        ChannelFeatureFlags {
            id: row.id,
            channel_id: row.channel_id,
            text_enabled: row.text_enabled,
            file_upload_enabled: row.file_upload_enabled,
            voice_group_enabled: row.voice_group_enabled,
            video_group_enabled: row.video_group_enabled,
            threads_enabled: row.threads_enabled,
            reactions_enabled: row.reactions_enabled,
        }
    }
}
