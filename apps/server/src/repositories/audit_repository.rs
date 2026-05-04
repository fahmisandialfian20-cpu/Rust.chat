use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::audit::AuditEntry;
use crate::error::AppError;

#[derive(sqlx::FromRow)]
struct SqlxAuditEntry {
    id: Uuid,
    user_id: Option<Uuid>,
    space_id: Option<Uuid>,
    action: String,
    target_user_id: Option<Uuid>,
    target_space_id: Option<Uuid>,
    target_channel_id: Option<Uuid>,
    metadata: serde_json::Value,
    ip_address: Option<String>,
    user_agent: Option<String>,
    created_at: OffsetDateTime,
}

impl From<SqlxAuditEntry> for AuditEntry {
    fn from(row: SqlxAuditEntry) -> Self {
        Self {
            id: row.id,
            user_id: row.user_id,
            space_id: row.space_id,
            action: row.action,
            target_user_id: row.target_user_id,
            target_space_id: row.target_space_id,
            target_channel_id: row.target_channel_id,
            metadata: row.metadata,
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            created_at: row.created_at,
        }
    }
}

#[derive(Clone)]
pub struct AuditRepository {
    pool: PgPool,
}

impl AuditRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, entry: &AuditEntry) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, space_id, action, target_user_id, target_space_id, target_channel_id, metadata, ip_address, user_agent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(entry.id)
        .bind(entry.user_id)
        .bind(entry.space_id)
        .bind(&entry.action)
        .bind(entry.target_user_id)
        .bind(entry.target_space_id)
        .bind(entry.target_channel_id)
        .bind(&entry.metadata)
        .bind(&entry.ip_address)
        .bind(&entry.user_agent)
        .bind(entry.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<AuditEntry>, AppError> {
        let rows = sqlx::query_as::<_, SqlxAuditEntry>(
            r#"
            SELECT id, user_id, space_id, action, target_user_id, target_space_id, target_channel_id, metadata, ip_address, user_agent, created_at
            FROM audit_logs
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_actor(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditEntry>, AppError> {
        let rows = sqlx::query_as::<_, SqlxAuditEntry>(
            r#"
            SELECT id, user_id, space_id, action, target_user_id, target_space_id, target_channel_id, metadata, ip_address, user_agent, created_at
            FROM audit_logs
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(Into::into).collect())
    }
}
