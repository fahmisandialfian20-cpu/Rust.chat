use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::role::Role;
use crate::error::AppError;

pub struct RoleRepository {
    pool: PgPool,
}

impl RoleRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        space_id: Uuid,
        name: &str,
        is_default: bool,
    ) -> Result<Role, AppError> {
        let row = sqlx::query_as::<_, SqlxRole>(
            r#"
            INSERT INTO roles (id, space_id, name, is_default, permissions, created_at, updated_at)
            VALUES ($1, $2, $3, $4, '{}', $5, $5)
            RETURNING id, space_id, name, is_default, permissions, created_at, updated_at
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(space_id)
        .bind(name)
        .bind(is_default)
        .bind(OffsetDateTime::now_utc())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(row.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Role, AppError> {
        let row = sqlx::query_as::<_, SqlxRole>(
            r#"
            SELECT id, space_id, name, is_default, permissions, created_at, updated_at
            FROM roles WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Role not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn find_by_space(&self, space_id: Uuid) -> Result<Vec<Role>, AppError> {
        let rows = sqlx::query_as::<_, SqlxRole>(
            r#"
            SELECT id, space_id, name, is_default, permissions, created_at, updated_at
            FROM roles
            WHERE space_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(space_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn update(&self, id: Uuid, name: &str) -> Result<Role, AppError> {
        let row = sqlx::query_as::<_, SqlxRole>(
            r#"
            UPDATE roles SET name = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, space_id, name, is_default, permissions, created_at, updated_at
            "#,
        )
        .bind(name)
        .bind(OffsetDateTime::now_utc())
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("Role not found".to_string()),
            _ => AppError::InternalServerError(e.to_string()),
        })?;

        Ok(row.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM roles WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Role not found".to_string()));
        }

        Ok(())
    }

    pub async fn set_permissions(
        &self,
        role_id: Uuid,
        permission_keys: &[String],
        allowed: bool,
    ) -> Result<(), AppError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        sqlx::query("DELETE FROM role_permissions WHERE role_id = $1")
            .bind(role_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        for key in permission_keys {
            sqlx::query(
                r#"
                INSERT INTO role_permissions (id, role_id, permission_key, allowed, created_at)
                VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(Uuid::now_v7())
            .bind(role_id)
            .bind(key)
            .bind(allowed)
            .bind(OffsetDateTime::now_utc())
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn assign_role_to_member(
        &self,
        membership_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO member_roles (id, membership_id, role_id, assigned_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (membership_id, role_id) DO NOTHING
            "#,
        )
        .bind(Uuid::now_v7())
        .bind(membership_id)
        .bind(role_id)
        .bind(OffsetDateTime::now_utc())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    pub async fn remove_role_from_member(
        &self,
        membership_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), AppError> {
        let result =
            sqlx::query("DELETE FROM member_roles WHERE membership_id = $1 AND role_id = $2")
                .bind(membership_id)
                .bind(role_id)
                .execute(&self.pool)
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Role assignment not found".to_string()));
        }

        Ok(())
    }

    pub async fn find_by_space_and_member(
        &self,
        space_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<Role>, AppError> {
        let rows = sqlx::query_as::<_, SqlxRole>(
            r#"
            SELECT r.id, r.space_id, r.name, r.is_default, r.permissions, r.created_at, r.updated_at
            FROM roles r
            INNER JOIN member_roles mr ON r.id = mr.role_id
            INNER JOIN space_memberships sm ON mr.membership_id = sm.id
            WHERE sm.space_id = $1 AND sm.user_id = $2
            "#,
        )
        .bind(space_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn find_membership_id(
        &self,
        space_id: Uuid,
        user_id: Uuid,
    ) -> Result<Uuid, AppError> {
        let result = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT id FROM space_memberships
            WHERE space_id = $1 AND user_id = $2
            "#,
        )
        .bind(space_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        match result {
            Some(id) => Ok(id),
            None => Err(AppError::NotFound("Membership not found".to_string())),
        }
    }

    pub async fn get_permission_keys(&self, role_id: Uuid) -> Result<Vec<String>, AppError> {
        let rows = sqlx::query_scalar::<_, String>(
            r#"
            SELECT permission_key FROM role_permissions
            WHERE role_id = $1 AND allowed = true
            "#,
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rows)
    }
}

#[derive(sqlx::FromRow)]
struct SqlxRole {
    id: Uuid,
    space_id: Uuid,
    name: String,
    is_default: bool,
    permissions: serde_json::Value,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl From<SqlxRole> for Role {
    fn from(row: SqlxRole) -> Self {
        Role {
            id: row.id,
            space_id: row.space_id,
            name: row.name,
            is_default: row.is_default,
            permissions: row.permissions,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
