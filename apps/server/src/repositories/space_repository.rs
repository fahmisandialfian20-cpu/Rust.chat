use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::membership::SpaceMembership;
use crate::domain::space::{Space, SpaceVisibility};
use crate::error::AppError;

pub struct SpaceRepository {
    pool: PgPool,
}

impl SpaceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        name: String,
        slug: String,
        description: Option<String>,
        created_by: Uuid,
        visibility: SpaceVisibility,
    ) -> Result<Space, AppError> {
        let row = sqlx::query_as::<_, SqlxSpace>(
            r#"
            INSERT INTO spaces (id, name, slug, description, created_by, visibility, settings, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, '{}', $7, $7)
            RETURNING id, name, slug, description, icon_object_id, created_by, visibility, settings, created_at, updated_at
            "#
        )
        .bind(Uuid::now_v7())
        .bind(&name)
        .bind(&slug)
        .bind(&description)
        .bind(created_by)
        .bind(visibility.to_string())
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Space, AppError> {
        let row = sqlx::query_as::<_, SqlxSpace>(
            r#"
            SELECT id, name, slug, description, icon_object_id, created_by, visibility, settings, created_at, updated_at
            FROM spaces WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Space not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<Space, AppError> {
        let row = sqlx::query_as::<_, SqlxSpace>(
            r#"
            SELECT id, name, slug, description, icon_object_id, created_by, visibility, settings, created_at, updated_at
            FROM spaces WHERE slug = $1
            "#
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Space not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<Space>, AppError> {
        let rows = sqlx::query_as::<_, SqlxSpace>(
            r#"
            SELECT id, name, slug, description, icon_object_id, created_by, visibility, settings, created_at, updated_at
            FROM spaces
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn find_by_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Space>, AppError> {
        let rows = sqlx::query_as::<_, SqlxSpace>(
            r#"
            SELECT s.id, s.name, s.slug, s.description, s.icon_object_id, s.created_by, s.visibility, s.settings, s.created_at, s.updated_at
            FROM spaces s
            INNER JOIN space_memberships sm ON s.id = sm.space_id
            WHERE sm.user_id = $1
            ORDER BY s.created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
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
        description: Option<String>,
        visibility: Option<String>,
    ) -> Result<Space, AppError> {
        let mut updates = Vec::new();
        let mut bind_idx = 1;

        if name.is_some() {
            updates.push(format!("name = ${}", bind_idx));
            bind_idx += 1;
        }
        if description.is_some() {
            updates.push(format!("description = ${}", bind_idx));
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
            "UPDATE spaces SET {} WHERE id = ${} RETURNING id, name, slug, description, icon_object_id, created_by, visibility, settings, created_at, updated_at",
            updates.join(", "),
            bind_idx
        );

        let mut query_builder = sqlx::query_as::<_, SqlxSpace>(&query);

        if let Some(ref n) = name {
            query_builder = query_builder.bind(n);
        }
        if let Some(ref d) = description {
            query_builder = query_builder.bind(d);
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
                sqlx::Error::RowNotFound => AppError::NotFound("Space not found".to_string()),
                _ => AppError::InternalServerError(e.to_string()),
            })?;

        Ok(row.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM spaces WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Space not found".to_string()));
        }

        Ok(())
    }

    pub async fn add_member(
        &self,
        space_id: Uuid,
        user_id: Uuid,
        nickname: Option<String>,
    ) -> Result<SpaceMembership, AppError> {
        let row = sqlx::query_as::<_, SqlxMembership>(
            r#"
            INSERT INTO space_memberships (id, space_id, user_id, nickname, settings, created_at, updated_at)
            VALUES ($1, $2, $3, $4, '{}', $5, $5)
            ON CONFLICT (space_id, user_id) DO UPDATE SET nickname = EXCLUDED.nickname, updated_at = EXCLUDED.updated_at
            RETURNING id, space_id, user_id, nickname, settings, created_at, updated_at
            "#
        )
        .bind(Uuid::now_v7())
        .bind(space_id)
        .bind(user_id)
        .bind(&nickname)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn remove_member(&self, space_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result =
            sqlx::query("DELETE FROM space_memberships WHERE space_id = $1 AND user_id = $2")
                .bind(space_id)
                .bind(user_id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Membership not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_members(
        &self,
        space_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<SpaceMembership>, AppError> {
        let rows = sqlx::query_as::<_, SqlxMembership>(
            r#"
            SELECT id, space_id, user_id, nickname, settings, created_at, updated_at
            FROM space_memberships
            WHERE space_id = $1
            ORDER BY created_at ASC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(space_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_member(
        &self,
        space_id: Uuid,
        user_id: Uuid,
    ) -> Result<SpaceMembership, AppError> {
        let row = sqlx::query_as::<_, SqlxMembership>(
            r#"
            SELECT id, space_id, user_id, nickname, settings, created_at, updated_at
            FROM space_memberships
            WHERE space_id = $1 AND user_id = $2
            "#,
        )
        .bind(space_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Membership not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn is_member(&self, space_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(SELECT 1 FROM space_memberships WHERE space_id = $1 AND user_id = $2)
            "#,
        )
        .bind(space_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    pub async fn slug_exists(&self, slug: &str) -> Result<bool, AppError> {
        let result =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM spaces WHERE slug = $1)")
                .bind(slug)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }
}

#[derive(sqlx::FromRow)]
struct SqlxSpace {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    icon_object_id: Option<Uuid>,
    created_by: Uuid,
    visibility: String,
    settings: serde_json::Value,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<SqlxSpace> for Space {
    fn from(row: SqlxSpace) -> Self {
        Space {
            id: row.id,
            name: row.name,
            slug: row.slug,
            description: row.description,
            icon_object_id: row.icon_object_id,
            created_by: row.created_by,
            visibility: row.visibility.parse().unwrap_or_default(),
            settings: row.settings,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct SqlxMembership {
    id: Uuid,
    space_id: Uuid,
    user_id: Uuid,
    nickname: Option<String>,
    settings: serde_json::Value,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<SqlxMembership> for SpaceMembership {
    fn from(row: SqlxMembership) -> Self {
        SpaceMembership {
            id: row.id,
            space_id: row.space_id,
            user_id: row.user_id,
            nickname: row.nickname,
            settings: row.settings,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
