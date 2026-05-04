mod common;

use sqlx::PgPool;
use uuid::Uuid;
use rust_chat_server::error::AppError;
use rust_chat_server::permissions::{PermissionKey, PermissionService};

async fn setup_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set for tests");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let _ = sqlx::query("DROP SCHEMA IF EXISTS public CASCADE")
        .execute(&pool)
        .await;
    sqlx::query("CREATE SCHEMA IF NOT EXISTS public")
        .execute(&pool)
        .await
        .expect("Failed to create schema");
    sqlx::query("SET search_path TO public")
        .execute(&pool)
        .await
        .expect("Failed to set search_path");
    let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS citext")
        .execute(&pool)
        .await;
    let _ = sqlx::query("CREATE EXTENSION IF NOT EXISTS pgcrypto")
        .execute(&pool)
        .await;

    let migrations_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    let migrator = sqlx::migrate::Migrator::new(migrations_dir)
        .await
        .expect("Failed to create migrator");
    migrator
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

async fn create_user(pool: &PgPool, username: &str) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, status) VALUES ($1, $2, $3, 'active')",
    )
    .bind(id)
    .bind(username)
    .bind("test-hash")
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn create_space(pool: &PgPool, name: &str, created_by: Uuid) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO spaces (id, name, slug, created_by) VALUES ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(name)
    .bind(name.to_lowercase())
    .bind(created_by)
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn create_channel(pool: &PgPool, space_id: Uuid, name: &str, created_by: Uuid) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO channels (id, space_id, name, slug, kind, visibility, created_by) VALUES ($1, $2, $3, $4, 'text', 'public', $5)",
    )
    .bind(id)
    .bind(space_id)
    .bind(name)
    .bind(name.to_lowercase())
    .bind(created_by)
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn make_member(pool: &PgPool, user_id: Uuid, space_id: Uuid) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO space_memberships (id, space_id, user_id) VALUES ($1, $2, $3)",
    )
    .bind(id)
    .bind(space_id)
    .bind(user_id)
    .execute(pool)
    .await
    .unwrap();
    id
}

async fn create_role(pool: &PgPool, space_id: Uuid, name: &str) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query("INSERT INTO roles (id, space_id, name) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(space_id)
        .bind(name)
        .execute(pool)
        .await
        .unwrap();
    id
}

async fn grant_permission(pool: &PgPool, role_id: Uuid, permission: &str) {
    sqlx::query(
        "INSERT INTO role_permissions (role_id, permission_key, allowed) VALUES ($1, $2, true)",
    )
    .bind(role_id)
    .bind(permission)
    .execute(pool)
    .await
    .unwrap();
}

async fn assign_role(pool: &PgPool, membership_id: Uuid, role_id: Uuid) {
    sqlx::query("INSERT INTO member_roles (membership_id, role_id) VALUES ($1, $2)")
        .bind(membership_id)
        .bind(role_id)
        .execute(pool)
        .await
        .unwrap();
}

async fn setup_permission_test() -> (PermissionService, PgPool, Uuid, Uuid, Uuid) {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "testuser").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;
    let service = PermissionService::new(pool.clone());
    (service, pool, user_id, space_id, channel_id)
}

#[tokio::test]
async fn hoster_bypass_any_permission() {
    let (service, pool, user_id, _, _) = setup_permission_test().await;

    sqlx::query(
        "INSERT INTO instance_settings (id, owner_user_id, instance_name) VALUES (1, $1, 'test-instance')",
    )
    .bind(user_id)
    .execute(&pool)
    .await
    .unwrap();

    let result = service
        .check(user_id, PermissionKey::ManageInstance, None, None)
        .await;

    assert!(result.is_ok(), "Hoster should bypass all permission checks");
}

#[tokio::test]
async fn non_member_denied() {
    let (service, _pool, user_id, space_id, _) = setup_permission_test().await;

    let result = service
        .check(user_id, PermissionKey::ViewSpace, Some(space_id), None)
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Non-member should get Forbidden"
    );
}

#[tokio::test]
async fn role_allow_channel_deny_returns_denied() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "send_messages").await;
    assign_role(&pool, membership_id, role_id).await;

    sqlx::query(
        "INSERT INTO channel_permission_overrides (channel_id, role_id, permission_key, denied) VALUES ($1, $2, $3, true)",
    )
    .bind(channel_id)
    .bind(role_id)
    .bind("send_messages")
    .execute(&pool)
    .await
    .unwrap();

    let result = service
        .check(
            user_id,
            PermissionKey::SendMessages,
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(reason) if reason.contains("channel override")),
        "Channel deny should override role allow"
    );
}

#[tokio::test]
async fn feature_flag_disabled_returns_denied() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "send_files").await;
    assign_role(&pool, membership_id, role_id).await;

    sqlx::query(
        "INSERT INTO channel_feature_flags (channel_id, send_file_enabled) VALUES ($1, false)",
    )
    .bind(channel_id)
    .execute(&pool)
    .await
    .unwrap();

    let result = service
        .check(
            user_id,
            PermissionKey::SendFiles,
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(reason) if reason.contains("Feature not enabled")),
        "Disabled feature flag should deny permission"
    );
}

#[tokio::test]
async fn role_allow_grants_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    assign_role(&pool, membership_id, role_id).await;

    let result = service
        .check(
            user_id,
            PermissionKey::ReadMessages,
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(result.is_ok(), "Role with matching permission should allow");
}

#[tokio::test]
async fn has_any_permission_works() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "view_channel").await;
    assign_role(&pool, membership_id, role_id).await;

    let result = service
        .has_any_permission(
            user_id,
            &[PermissionKey::ViewChannel, PermissionKey::ManageChannels],
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(
        result.unwrap(),
        "has_any_permission should return true when at least one permission matches"
    );

    let result = service
        .has_any_permission(
            user_id,
            &[PermissionKey::ManageChannels],
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(
        !result.unwrap(),
        "has_any_permission should return false when no permission matches"
    );
}
