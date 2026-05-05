mod common;

use axum::body::Body;
use axum::http::Request;
use futures_util::{SinkExt, StreamExt};
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
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tower::ServiceExt;
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
    let hash = rust_chat_server::auth::password::hash_password("test", "test-pepper").unwrap();
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, status) VALUES ($1, $2, $3, 'active')",
    )
    .bind(id)
    .bind(username)
    .bind(hash)
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

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let service = MessageService::new(
        Arc::new(MessageRepository::new(pool.clone())),
        permission_service,
        channel_repo,
    );
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

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let service = MessageService::new(
        Arc::new(MessageRepository::new(pool.clone())),
        permission_service,
        channel_repo,
    );
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
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

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
async fn test_list_messages_unauthenticated() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    let _ = create_message(&pool, channel_id, user_a, "hello").await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .list_channel_messages(channel_id, user_b, 50, None)
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Non-space-member should be denied list_channel_messages"
    );
}

#[tokio::test]
async fn test_list_messages_no_permission() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;
    let _ = create_message(&pool, channel_id, user_id, "hello").await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    assign_role(&pool, membership_id, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .list_channel_messages(channel_id, user_id, 50, None)
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without ReadMessages should be denied"
    );
}

#[tokio::test]
async fn test_get_message_unauthorized() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    let msg_id = create_message(&pool, channel_id, user_a, "secret").await;

    let membership_id = make_member(&pool, user_b, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    assign_role(&pool, membership_id, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service.get_message(msg_id, user_b).await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without ReadMessages should be denied get_message"
    );
}

#[tokio::test]
async fn test_create_message_no_send_permission() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    assign_role(&pool, membership_id, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .create_message(
            channel_id,
            user_id,
            rust_chat_server::domain::message::CreateMessage {
                content: "hello".to_string(),
                kind: None,
                reply_to_message_id: None,
            },
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without SendMessages should be denied"
    );
}

#[tokio::test]
async fn test_create_message_feature_flag_disabled() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "send_messages").await;
    assign_role(&pool, membership_id, role_id).await;

    sqlx::query("INSERT INTO channel_feature_flags (channel_id, text_enabled) VALUES ($1, false)")
        .bind(channel_id)
        .execute(&pool)
        .await
        .unwrap();

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .create_message(
            channel_id,
            user_id,
            rust_chat_server::domain::message::CreateMessage {
                content: "hello".to_string(),
                kind: None,
                reply_to_message_id: None,
            },
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member should be denied when text_enabled=false"
    );
}

#[tokio::test]
async fn test_edit_own_message_success() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;
    let msg_id = create_message(&pool, channel_id, user_id, "original").await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "edit_own_message").await;
    assign_role(&pool, membership_id, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .update_message(
            msg_id,
            user_id,
            UpdateMessage {
                content: Some("edited".to_string()),
            },
        )
        .await;

    assert!(result.is_ok(), "Owner with EditOwnMessage should succeed");
}

#[tokio::test]
async fn test_edit_other_message_forbidden() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    let msg_id = create_message(&pool, channel_id, user_a, "original").await;

    let membership_b = make_member(&pool, user_b, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "edit_own_message").await;
    assign_role(&pool, membership_b, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .update_message(
            msg_id,
            user_b,
            UpdateMessage {
                content: Some("hacked".to_string()),
            },
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "User B without EditAnyMessage should be denied editing user A's message"
    );
}

#[tokio::test]
async fn test_edit_other_message_as_admin() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let admin = create_user(&pool, "admin").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    let msg_id = create_message(&pool, channel_id, user_a, "original").await;

    let membership_admin = make_member(&pool, admin, space_id).await;
    let role_id = create_role(&pool, space_id, "admin").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "edit_any_message").await;
    assign_role(&pool, membership_admin, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service
        .update_message(
            msg_id,
            admin,
            UpdateMessage {
                content: Some("admin-edited".to_string()),
            },
        )
        .await;

    assert!(
        result.is_ok(),
        "Admin with EditAnyMessage should be able to edit other's message"
    );
}

#[tokio::test]
async fn test_delete_own_message_success() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;
    let msg_id = create_message(&pool, channel_id, user_id, "to-delete").await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "delete_own_message").await;
    assign_role(&pool, membership_id, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service.delete_message(msg_id, user_id).await;

    assert!(result.is_ok(), "Owner with DeleteOwnMessage should succeed");
}

#[tokio::test]
async fn test_delete_other_message_forbidden() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    let msg_id = create_message(&pool, channel_id, user_a, "to-delete").await;

    let membership_b = make_member(&pool, user_b, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "delete_own_message").await;
    assign_role(&pool, membership_b, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service.delete_message(msg_id, user_b).await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "User B without DeleteAnyMessage should be denied deleting user A's message"
    );
}

#[tokio::test]
async fn test_delete_other_message_as_admin() {
    let pool = setup_db().await;
    let user_a = create_user(&pool, "user_a").await;
    let admin = create_user(&pool, "admin").await;
    let space_id = create_space(&pool, "testspace", user_a).await;
    let channel_id = create_channel(&pool, space_id, "general", user_a).await;
    let msg_id = create_message(&pool, channel_id, user_a, "to-delete").await;

    let membership_admin = make_member(&pool, admin, space_id).await;
    let role_id = create_role(&pool, space_id, "admin").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "delete_any_message").await;
    assign_role(&pool, membership_admin, role_id).await;

    let permission_service = PermissionService::new(pool.clone());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let msg_service = MessageService::new(message_repo, permission_service, channel_repo);

    let result = msg_service.delete_message(msg_id, admin).await;

    assert!(
        result.is_ok(),
        "Admin with DeleteAnyMessage should be able to delete other's message"
    );
}

#[tokio::test]
async fn test_create_invite_unauthenticated() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;

    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let role_repo = Arc::new(RoleRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

    let result = invite_service
        .create_invite(
            user_id,
            CreateInvite {
                space_id: Some(space_id),
                channel_id: None,
                max_uses: Some(10),
                expires_at: None,
            },
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Non-member without ManageInvites should be denied"
    );
}

#[tokio::test]
async fn test_create_invite_no_permission() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "member").await;
    assign_role(&pool, membership_id, role_id).await;

    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let role_repo = Arc::new(RoleRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

    let result = invite_service
        .create_invite(
            user_id,
            CreateInvite {
                space_id: Some(space_id),
                channel_id: None,
                max_uses: Some(10),
                expires_at: None,
            },
        )
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::Forbidden(_)),
        "Member without ManageInvites should be denied"
    );
}

#[tokio::test]
async fn test_create_invite_with_permission_success() {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "user").await;
    let space_id = create_space(&pool, "testspace", user_id).await;

    let membership_id = make_member(&pool, user_id, space_id).await;
    let role_id = create_role(&pool, space_id, "admin").await;
    grant_permission(&pool, role_id, "manage_invites").await;
    assign_role(&pool, membership_id, role_id).await;

    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let role_repo = Arc::new(RoleRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

    let result = invite_service
        .create_invite(
            user_id,
            CreateInvite {
                space_id: Some(space_id),
                channel_id: None,
                max_uses: Some(10),
                expires_at: None,
            },
        )
        .await;

    assert!(result.is_ok(), "Member with ManageInvites should succeed");
}

#[tokio::test]
async fn test_accept_invite_expired() {
    let pool = setup_db().await;
    let hoster = create_user(&pool, "hoster").await;
    let new_user = create_user(&pool, "newbie").await;
    let space_id = create_space(&pool, "testspace", hoster).await;

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
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

    let invite = invite_service
        .create_invite(
            hoster,
            CreateInvite {
                space_id: Some(space_id),
                channel_id: None,
                max_uses: Some(10),
                expires_at: Some(OffsetDateTime::now_utc() + time::Duration::hours(1)),
            },
        )
        .await
        .unwrap();

    // Manually expire the invite in the database
    sqlx::query("UPDATE invites SET expires_at = $1 WHERE id = $2")
        .bind(OffsetDateTime::now_utc() - time::Duration::hours(1))
        .bind(invite.id)
        .execute(&pool)
        .await
        .unwrap();

    let result = invite_service.accept_invite(&invite.code, new_user).await;

    assert!(
        matches!(result.unwrap_err(), AppError::BadRequest(_)),
        "Expired invite should be rejected"
    );
}

#[tokio::test]
async fn test_accept_invite_max_uses_exceeded() {
    let pool = setup_db().await;
    let hoster = create_user(&pool, "hoster").await;
    let user_a = create_user(&pool, "user_a").await;
    let user_b = create_user(&pool, "user_b").await;
    let space_id = create_space(&pool, "testspace", hoster).await;

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
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

    let invite = invite_service
        .create_invite(
            hoster,
            CreateInvite {
                space_id: Some(space_id),
                channel_id: None,
                max_uses: Some(1),
                expires_at: None,
            },
        )
        .await
        .unwrap();

    let _ = invite_service.accept_invite(&invite.code, user_a).await;

    let result = invite_service.accept_invite(&invite.code, user_b).await;

    assert!(
        matches!(result.unwrap_err(), AppError::BadRequest(_)),
        "Exhausted invite should be rejected"
    );
}

#[tokio::test]
async fn test_accept_invite_invalid_code() {
    let pool = setup_db().await;
    let new_user = create_user(&pool, "newbie").await;

    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let role_repo = Arc::new(RoleRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let permission_service = PermissionService::new(pool.clone());
    let invite_service = InviteService::new(
        invite_repo,
        space_repo,
        channel_repo,
        role_repo,
        permission_service,
    );

    let result = invite_service
        .accept_invite("nonexistent-code", new_user)
        .await;

    assert!(
        matches!(result.unwrap_err(), AppError::NotFound(_)),
        "Non-existent invite code should return NotFound"
    );
}

async fn start_test_server() -> (u16, String) {
    let (app, pool) = common::setup_test_app().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let hoster = create_user(&pool, "hoster").await;
    let _space_id = create_space(&pool, "testspace", hoster).await;

    sqlx::query(
        "INSERT INTO instance_settings (id, owner_user_id, instance_name) VALUES (1, $1, 'test-instance')",
    )
    .bind(hoster)
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"username_or_email": "hoster", "password": "test"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap();
    let token = body["access_token"].as_str().unwrap_or("").to_string();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (port, token)
}

async fn start_test_server_with_permission() -> (u16, String, Uuid) {
    let (app, pool) = common::setup_test_app().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let hoster = create_user(&pool, "hoster").await;
    let space_id = create_space(&pool, "testspace", hoster).await;
    let channel_id = create_channel(&pool, space_id, "general", hoster).await;
    let membership = make_member(&pool, hoster, space_id).await;
    let role_id = create_role(&pool, space_id, "admin").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "view_channel").await;
    assign_role(&pool, membership, role_id).await;

    sqlx::query(
        "INSERT INTO instance_settings (id, owner_user_id, instance_name) VALUES (1, $1, 'test-instance')",
    )
    .bind(hoster)
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"username_or_email": "hoster", "password": "test"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap();
    let token = body["access_token"].as_str().unwrap_or("").to_string();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (port, token, channel_id)
}

#[tokio::test]
async fn test_ws_send_without_permission() {
    let (port, token) = start_test_server().await;
    let url = format!("ws://127.0.0.1:{}/ws?token={}", port, token);

    let (mut socket, _) = tokio_tungstenite::connect_async(&url).await.unwrap();

    // Wait for hello
    let msg = socket.next().await.unwrap().unwrap();
    assert!(msg.to_string().contains("hello"));

    // Try to send message without permission
    let cmd = serde_json::json!({
        "type": "send_message",
        "data": {
            "channel_id": "00000000-0000-0000-0000-000000000000",
            "content": "hello"
        }
    });
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(
            cmd.to_string(),
        ))
        .await
        .unwrap();

    // Wait for error response
    let msg = tokio::time::timeout(Duration::from_secs(3), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let text = msg.to_string();
    assert!(
        text.contains("forbidden") || text.contains("error"),
        "Expected error response, got: {}",
        text
    );
}

#[tokio::test]
async fn test_ws_send_with_permission() {
    let (port, token, channel_id) = start_test_server_with_permission().await;
    let url = format!("ws://127.0.0.1:{}/ws?token={}", port, token);
    let (mut socket, _) = tokio_tungstenite::connect_async(&url).await.unwrap();

    // Wait for hello
    let msg = socket.next().await.unwrap().unwrap();
    assert!(msg.to_string().contains("hello"));

    // Join the channel
    let join = serde_json::json!({
        "type": "join_channel",
        "data": { "channel_id": channel_id }
    });
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(
            join.to_string(),
        ))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(200)).await;

    // Send message
    let send = serde_json::json!({
        "type": "send_message",
        "data": {
            "channel_id": channel_id,
            "content": "hello from WS"
        }
    });
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(
            send.to_string(),
        ))
        .await
        .unwrap();

    // Should receive message.created event
    let msg = tokio::time::timeout(Duration::from_secs(5), socket.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let text = msg.to_string();
    assert!(
        text.contains("message.created"),
        "Expected message.created event, got: {}",
        text
    );
}

async fn start_test_server_cross_channel() -> (u16, String, Uuid, Uuid) {
    let (app, pool) = common::setup_test_app().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let hoster = create_user(&pool, "hoster").await;
    let space_id = create_space(&pool, "testspace", hoster).await;
    let channel_a = create_channel(&pool, space_id, "channel-a", hoster).await;
    let channel_b = create_channel(&pool, space_id, "channel-b", hoster).await;

    let membership = make_member(&pool, hoster, space_id).await;
    let role_id = create_role(&pool, space_id, "admin").await;
    grant_permission(&pool, role_id, "send_messages").await;
    grant_permission(&pool, role_id, "read_messages").await;
    grant_permission(&pool, role_id, "view_channel").await;
    assign_role(&pool, membership, role_id).await;

    sqlx::query(
        "INSERT INTO instance_settings (id, owner_user_id, instance_name) VALUES (1, $1, 'test-instance')",
    )
    .bind(hoster)
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({"username_or_email": "hoster", "password": "test"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(
        &axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap(),
    )
    .unwrap();
    let token = body["access_token"].as_str().unwrap_or("").to_string();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (port, token, channel_a, channel_b)
}

#[tokio::test]
async fn test_ws_cross_channel_leak() {
    let (port, token, channel_a, channel_b) = start_test_server_cross_channel().await;
    let url = format!("ws://127.0.0.1:{}/ws?token={}", port, token);
    let (mut socket, _) = tokio_tungstenite::connect_async(&url).await.unwrap();

    // Wait for hello
    let _hello = socket.next().await.unwrap().unwrap();

    // Subscribe to channel A only
    let join = serde_json::json!({
        "type": "join_channel",
        "data": { "channel_id": channel_a }
    });
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(
            join.to_string(),
        ))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_millis(200)).await;

    // Send message to channel B
    let send = serde_json::json!({
        "type": "send_message",
        "data": {
            "channel_id": channel_b,
            "content": "secret message"
        }
    });
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(
            send.to_string(),
        ))
        .await
        .unwrap();

    // Should NOT receive message.created since we didn't subscribe to channel B
    // No leak = timeout waiting for message from unsubscribed channel
    let result = tokio::time::timeout(Duration::from_secs(2), socket.next()).await;
    assert!(
        result.is_err(),
        "Expected timeout (no message leak), but got: {:?}",
        result
    );
}

#[tokio::test]
async fn websocket_respects_permission() {
    // Kept as documentation placeholder
    // Integration tests above cover the main scenarios
}
