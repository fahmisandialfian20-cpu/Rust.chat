use sqlx::PgPool;
use uuid::Uuid;
use time::OffsetDateTime;
use sha2::{Sha256, Digest};

use crate::domain::invite::Invite;
use crate::error::AppError;

pub struct InviteRepository {
    pool: PgPool,
}

impl InviteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        space_id: Option<Uuid>,
        channel_id: Option<Uuid>,
        created_by: Uuid,
        max_uses: Option<i32>,
        expires_at: Option<OffsetDateTime>,
    ) -> Result<Invite, AppError> {
        let code = Self::generate_code();

        let row = sqlx::query_as::<_, SqlxInvite>(
            r#"
            INSERT INTO invites (id, code_hash, space_id, channel_id, created_by, max_uses, used_count, expires_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, 0, $7, $8)
            RETURNING id, code, code_hash, space_id, channel_id, created_by, max_uses, used_count, expires_at, created_at
            "#
        )
        .bind(Uuid::now_v7())
        .bind(Self::hash_code(&code))
        .bind(&space_id)
        .bind(&channel_id)
        .bind(created_by)
        .bind(&max_uses)
        .bind(&expires_at)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(Invite {
            id: row.id,
            code: row.code,
            space_id: row.space_id,
            channel_id: row.channel_id,
            created_by: row.created_by,
            max_uses: row.max_uses,
            used_count: row.used_count,
            expires_at: row.expires_at,
            created_at: row.created_at,
        })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Invite, AppError> {
        let row = sqlx::query_as::<_, SqlxInvite>(
            r#"
            SELECT id, code, code_hash, space_id, channel_id, created_by, max_uses, used_count, expires_at, created_at
            FROM invites WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Invite not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(Invite {
            id: row.id,
            code: row.code,
            space_id: row.space_id,
            channel_id: row.channel_id,
            created_by: row.created_by,
            max_uses: row.max_uses,
            used_count: row.used_count,
            expires_at: row.expires_at,
            created_at: row.created_at,
        })
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Invite, AppError> {
        let code_hash = Self::hash_code(code);
        let row = sqlx::query_as::<_, SqlxInvite>(
            r#"
            SELECT id, code, code_hash, space_id, channel_id, created_by, max_uses, used_count, expires_at, created_at
            FROM invites WHERE code_hash = $1
            "#
        )
        .bind(code_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Invite not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(Invite {
            id: row.id,
            code: row.code,
            space_id: row.space_id,
            channel_id: row.channel_id,
            created_by: row.created_by,
            max_uses: row.max_uses,
            used_count: row.used_count,
            expires_at: row.expires_at,
            created_at: row.created_at,
        })
    }

    pub async fn find_by_space(&self, space_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Invite>, AppError> {
        let rows = sqlx::query_as::<_, SqlxInvite>(
            r#"
            SELECT id, code, code_hash, space_id, channel_id, created_by, max_uses, used_count, expires_at, created_at
            FROM invites
            WHERE space_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(space_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| Invite {
            id: r.id,
            code: r.code,
            space_id: r.space_id,
            channel_id: r.channel_id,
            created_by: r.created_by,
            max_uses: r.max_uses,
            used_count: r.used_count,
            expires_at: r.expires_at,
            created_at: r.created_at,
        }).collect())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM invites WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Invite not found".to_string()));
        }

        Ok(())
    }

    pub async fn increment_used_count(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE invites SET used_count = used_count + 1 WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn is_valid(&self, code: &str) -> Result<bool, AppError> {
        let invite = self.find_by_code(code).await?;
        
        if let Some(max_uses) = invite.max_uses {
            if invite.used_count >= max_uses {
                return Ok(false);
            }
        }

        if let Some(expires_at) = invite.expires_at {
            if expires_at < OffsetDateTime::now_utc() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn generate_code() -> String {
        uuid::Uuid::new_v4().to_string().replace('-', "")
    }

    fn hash_code(code: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(code.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[derive(sqlx::FromRow)]
struct SqlxInvite {
    id: Uuid,
    code: String,
    code_hash: String,
    space_id: Option<Uuid>,
    channel_id: Option<Uuid>,
    created_by: Uuid,
    max_uses: Option<i32>,
    used_count: i32,
    expires_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
}