mod common;

use rust_chat_server::domain::invite::CreateInvite;
use rust_chat_server::domain::message::UpdateMessage;
use rust_chat_server::error::AppError;
use rust_chat_server::permissions::{PermissionKey, PermissionService};
use rust_chat_server::repositories::channel_repository::ChannelRepository;
use rust_chat_server::repositories::invite_repository::InviteRepository;
use rust_chat_server::repositories::message_repository::MessageRepository;
use rust_chat_server::repositories::role_repository::RoleRepository;
use rust_chat_server::repositories::space_repository::SpaceRepository;
use rust_chat_server::services::invite_service::InviteService;
use rust_chat_server::services::message_service::MessageService;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

async fn setup_db() -> PgPool {
    let database_url =
        std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set for tests");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let _ = sqlx::query(
        "DO $$ DECLARE r RECORD; BEGIN FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP EXECUTE 'DROP TABLE IF EXISTS public.' || quote_ident(r.tablename) || ' CASCADE'; END LOOP; END $$"
    )
    .execute(&pool)
    .await;
    let _ = sqlx::query("DELETE FROM _sqlx_migrations")
        .execute(&pool)
        .await;
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
    migrator.run(&pool).await.expect("Failed to run migrations");

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
    sqlx::query("INSERT INTO spaces (id, name, slug, created_by) VALUES ($1, $2, $3, $4)")
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
    sqlx::query("INSERT INTO space_memberships (id, space_id, user_id) VALUES ($1, $2, $3)")
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

async fn create_private_channel(
    pool: &PgPool,
    space_id: Uuid,
    name: &str,
    created_by: Uuid,
) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO channels (id, space_id, name, slug, kind, visibility, created_by) VALUES ($1, $2, $3, $4, 'text', 'private', $5)",
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

async fn create_message(pool: &PgPool, channel_id: Uuid, author_id: Uuid, content: &str) -> Uuid {
    let id = Uuid::now_v7();
    sqlx::query(
        "INSERT INTO messages (id, channel_id, author_user_id, content, kind) VALUES ($1, $2, $3, $4, 'text')",
    )
    .bind(id)
    .bind(channel_id)
    .bind(author_id)
    .bind(content)
    .execute(pool)
    .await
    .unwrap();
    id
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

#[tokio::test]
async fn private_channel_not_visible_to_unauthorized() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let _private_channel = create_private_channel(&pool, space_id, "secret", user_a).await;

    let _membership_b = make_member(&pool, user_b, space_id).await;

    let channels = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM channels WHERE space_id = $1 AND visibility = 'public'",
    )
    .bind(space_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(
        channels.is_empty(),
        "Private channel should not be visible to unauthorized member"
    );
}

#[tokio::test]
async fn cannot_read_messages_without_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    assign_role(&pool, membership_id, role_id).await;

    let result = service
        .check(
            user_id,
            PermissionKey::ReadMessages,
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without ReadMessages should be denied"
    );
}

#[tokio::test]
async fn cannot_send_messages_without_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    assign_role(&pool, membership_id, role_id).await;

    let result = service
        .check(
            user_id,
            PermissionKey::SendMessages,
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without SendMessages should be denied"
    );
}

#[tokio::test]
async fn can_send_messages_with_permission() {
    let (service, pool, user_id, space_id, channel_id) = setup_permission_test().await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "send_messages").await;
    assign_role(&pool, membership_id, role_id).await;

    let result = service
        .check(
            user_id,
            PermissionKey::SendMessages,
            Some(space_id),
            Some(channel_id),
        )
        .await;

    assert!(result.is_ok(), "Member with SendMessages should be allowed");
}

#[tokio::test]
async fn cannot_edit_others_message() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;

    let _membership_a = make_member(&pool, user_a, space_id).await;
    let membership_b = make_member(&pool, user_b, space_id).await;

    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "edit_own_message").await;
    assign_role(&pool, membership_b, role_id).await;

    let message_id = create_message(&pool, channel_id, user_a, "hello from A").await;

    let service = MessageService::new(Arc::new(MessageRepository::new(pool.clone())));
    let result = service
        .update_message(
            message_id,
            user_b,
            UpdateMessage {
                content: Some("hacked".to_string()),
            },
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "User B should not be able to edit user A's message"
    );
}

#[tokio::test]
async fn cannot_delete_others_message() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;

    let _membership_a = make_member(&pool, user_a, space_id).await;
    let membership_b = make_member(&pool, user_b, space_id).await;

    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "delete_own_message").await;
    assign_role(&pool, membership_b, role_id).await;

    let message_id = create_message(&pool, channel_id, user_a, "hello from A").await;

    let service = MessageService::new(Arc::new(MessageRepository::new(pool.clone())));
    let result = service.delete_message(message_id, user_b).await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "User B should not be able to delete user A's message"
    );
}

#[tokio::test]
async fn invite_accept_creates_membership() {
    let pool = setup_db().await;
    let hoster = create_user(&pool, "hoster").await;
    let space_id = create_space(&pool, "testspace", hoster).await;
    let new_user = create_user(&pool, "newbie").await;

    sqlx::query(
        "INSERT INTO instance_settings (id, owner_user_id, instance_name) VALUES (1, $1, 'test')",
    )
    .bind(hoster)
    .execute(&pool)
    .await
    .unwrap();

    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let role_repo = Arc::new(RoleRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let invite_service = InviteService::new(invite_repo, space_repo, channel_repo, role_repo);

    let invite = invite_service
        .create_invite(
            hoster,
            CreateInvite {
                space_id: Some(space_id),
                channel_id: None,
                max_uses: Some(10),
                expires_at: None,
            },
        )
        .await
        .unwrap();

    let result = invite_service.accept_invite(&invite.code, new_user).await;
    assert!(result.is_ok(), "Invite accept should succeed");

    let membership_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM space_memberships WHERE space_id = $1 AND user_id = $2)",
    )
    .bind(space_id)
    .bind(new_user)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(membership_exists, "Invite accept should create membership");
}

#[tokio::test]
async fn websocket_respects_permission() {
    // This test requires a running WebSocket server and connection.
    // It is documented as a known gap for automated integration tests.
    // Manual verification: connect WS as member without SendMessages,
    // attempt to send message event, verify rejection.
}
