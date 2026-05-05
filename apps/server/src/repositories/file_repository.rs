use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::file_object::FileObject;
use crate::error::AppError;

#[derive(Clone)]
pub struct FileRepository {
    pool: PgPool,
}

impl FileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
        uploader_user_id: Uuid,
        filename: String,
        content_type: String,
        size_bytes: i64,
        storage_key: String,
    ) -> Result<FileObject, AppError> {
        let row = sqlx::query_as::<_, SqlxFileObject>(
            r#"
            INSERT INTO file_objects (id, space_id, channel_id, uploader_user_id, filename, content_type, size_bytes, storage_key, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, '{}', $9)
            RETURNING id, space_id, channel_id, uploader_user_id, filename, content_type, size_bytes, storage_key, metadata, created_at
            "#
        )
        .bind(Uuid::now_v7())
        .bind(space_id)
        .bind(channel_id)
        .bind(uploader_user_id)
        .bind(&filename)
        .bind(&content_type)
        .bind(size_bytes)
        .bind(&storage_key)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<FileObject, AppError> {
        let row = sqlx::query_as::<_, SqlxFileObject>(
            r#"
            SELECT id, space_id, channel_id, uploader_user_id, filename, content_type, size_bytes, storage_key, metadata, created_at
            FROM file_objects WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("File not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_channel(
        &self,
        channel_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<FileObject>, AppError> {
        let rows = sqlx::query_as::<_, SqlxFileObject>(
            r#"
            SELECT id, space_id, channel_id, uploader_user_id, filename, content_type, size_bytes, storage_key, metadata, created_at
            FROM file_objects
            WHERE channel_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(channel_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM file_objects WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("File not found".to_string()));
        }

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct SqlxFileObject {
    id: Uuid,
    space_id: Option<Uuid>,
    channel_id: Option<Uuid>,
    uploader_user_id: Uuid,
    filename: String,
    content_type: String,
    size_bytes: i64,
    storage_key: String,
    metadata: serde_json::Value,
    created_at: OffsetDateTime,
}

impl From<SqlxFileObject> for FileObject {
    fn from(row: SqlxFileObject) -> Self {
        FileObject {
            id: row.id,
            space_id: row.space_id,
            channel_id: row.channel_id,
            uploader_user_id: row.uploader_user_id,
            filename: row.filename,
            content_type: row.content_type,
            size_bytes: row.size_bytes,
            storage_key: row.storage_key,
            metadata: row.metadata,
            created_at: row.created_at,
        }
    }
}
