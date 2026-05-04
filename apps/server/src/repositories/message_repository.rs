use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::message::Message;
use crate::error::AppError;

pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        channel_id: Uuid,
        author_user_id: Uuid,
        content: String,
        kind: String,
        reply_to_message_id: Option<Uuid>,
    ) -> Result<Message, AppError> {
        let row = sqlx::query_as::<_, SqlxMessage>(
            r#"
            INSERT INTO messages (id, channel_id, author_user_id, content, kind, reply_to_message_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, channel_id, author_user_id, content, kind, reply_to_message_id, edited_at, deleted_at, created_at
            "#
        )
        .bind(Uuid::now_v7())
        .bind(channel_id)
        .bind(author_user_id)
        .bind(&content)
        .bind(&kind)
        .bind(reply_to_message_id)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Message, AppError> {
        let row = sqlx::query_as::<_, SqlxMessage>(
            r#"
            SELECT id, channel_id, author_user_id, content, kind, reply_to_message_id, edited_at, deleted_at, created_at
            FROM messages WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Message not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_channel(
        &self,
        channel_id: Uuid,
        limit: i64,
        before: Option<Uuid>,
    ) -> Result<Vec<Message>, AppError> {
        let query = if before.is_some() {
            sqlx::query_as::<_, SqlxMessage>(
                r#"
                SELECT id, channel_id, author_user_id, content, kind, reply_to_message_id, edited_at, deleted_at, created_at
                FROM messages 
                WHERE channel_id = $1 AND id < $2 AND deleted_at IS NULL
                ORDER BY created_at DESC
                LIMIT $3
                "#,
            )
        } else {
            sqlx::query_as::<_, SqlxMessage>(
                r#"
                SELECT id, channel_id, author_user_id, content, kind, reply_to_message_id, edited_at, deleted_at, created_at
                FROM messages 
                WHERE channel_id = $1 AND deleted_at IS NULL
                ORDER BY created_at DESC
                LIMIT $2
                "#,
            )
        };

        let rows = if let Some(before_id) = before {
            query
                .bind(channel_id)
                .bind(before_id)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?
        } else {
            query
                .bind(channel_id)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?
        };

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update(&self, id: Uuid, content: Option<String>) -> Result<Message, AppError> {
        let row = sqlx::query_as::<_, SqlxMessage>(
            r#"
            UPDATE messages 
            SET content = COALESCE($1, content), edited_at = $2
            WHERE id = $3 AND deleted_at IS NULL
            RETURNING id, channel_id, author_user_id, content, kind, reply_to_message_id, edited_at, deleted_at, created_at
            "#
        )
        .bind(&content)
        .bind(OffsetDateTime::now_utc())
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Message not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn soft_delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("UPDATE messages SET deleted_at = $1 WHERE id = $2")
            .bind(OffsetDateTime::now_utc())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Message not found".to_string()));
        }

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct SqlxMessage {
    id: Uuid,
    channel_id: Uuid,
    author_user_id: Uuid,
    content: String,
    kind: String,
    reply_to_message_id: Option<Uuid>,
    edited_at: Option<OffsetDateTime>,
    deleted_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
}

impl From<SqlxMessage> for Message {
    fn from(row: SqlxMessage) -> Self {
        Message {
            id: row.id,
            channel_id: row.channel_id,
            author_user_id: row.author_user_id,
            content: row.content,
            kind: row.kind,
            reply_to_message_id: row.reply_to_message_id,
            edited_at: row.edited_at,
            deleted_at: row.deleted_at,
            created_at: row.created_at,
        }
    }
}
