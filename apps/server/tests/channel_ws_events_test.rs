mod common;

use rust_chat_server::domain::channel::UpdateChannel;
use rust_chat_server::realtime::hub::RealtimeHub;
use rust_chat_server::repositories::channel_repository::ChannelRepository;
use rust_chat_server::services::channel_service::ChannelService;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

async fn setup_db() -> PgPool {
    let database_url =
        std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set for tests");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let _ = sqlx::query(
        "DO $$ DECLARE r RECORD; BEGIN FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP EXECUTE 'DROP TABLE IF EXISTS public.' || quote_ident(r.tablename) || ' CASCADE'; END LOOP; END $$",
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

async fn setup_channel_ws_test() -> (PgPool, Arc<RealtimeHub>, ChannelService, Uuid, Uuid, Uuid) {
    let pool = setup_db().await;
    let user_id = create_user(&pool, "hoster").await;
    let space_id = create_space(&pool, "testspace", user_id).await;
    let channel_id = create_channel(&pool, space_id, "general", user_id).await;

    let hub = Arc::new(RealtimeHub::default());
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let channel_service = ChannelService::new(channel_repo, hub.clone());

    (pool, hub, channel_service, user_id, space_id, channel_id)
}

#[tokio::test]
async fn test_channel_update_emits_event() {
    let (_pool, hub, service, _user_id, _space_id, channel_id) = setup_channel_ws_test().await;

    let mut rx = hub.subscribe(channel_id).await;

    let update = UpdateChannel {
        name: Some("updated-general".to_string()),
        topic: Some("Updated topic".to_string()),
        visibility: None,
        feature_flags: None,
    };

    service.update_channel(channel_id, update).await.unwrap();

    let msg = tokio::time::timeout(Duration::from_secs(3), rx.recv())
        .await
        .expect("Should receive channel.updated event within timeout")
        .expect("Broadcast should not be lagged");

    let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
    assert_eq!(parsed["type"], "channel.updated");
}

#[tokio::test]
async fn test_channel_delete_emits_event() {
    let (_pool, hub, service, _user_id, _space_id, channel_id) = setup_channel_ws_test().await;

    let mut rx = hub.subscribe(channel_id).await;

    service.delete_channel(channel_id).await.unwrap();

    let msg = tokio::time::timeout(Duration::from_secs(3), rx.recv())
        .await
        .expect("Should receive channel.deleted event within timeout")
        .expect("Broadcast should not be lagged");

    let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
    assert_eq!(parsed["type"], "channel.deleted");
}

#[tokio::test]
async fn test_channel_visibility_change_emits_event() {
    let (_pool, hub, service, _user_id, _space_id, channel_id) = setup_channel_ws_test().await;

    let mut rx = hub.subscribe(channel_id).await;

    let update = UpdateChannel {
        name: None,
        topic: None,
        visibility: Some("private".to_string()),
        feature_flags: None,
    };

    service.update_channel(channel_id, update).await.unwrap();

    let msg = tokio::time::timeout(Duration::from_secs(3), rx.recv())
        .await
        .expect("Should receive channel.updated event within timeout")
        .expect("Broadcast should not be lagged");

    let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
    assert_eq!(parsed["type"], "channel.updated");

    let msg = tokio::time::timeout(Duration::from_secs(3), rx.recv())
        .await
        .expect("Should receive channel.visibility_changed event within timeout")
        .expect("Broadcast should not be lagged");

    let parsed: serde_json::Value = serde_json::from_str(&msg).unwrap();
    assert_eq!(parsed["type"], "channel.visibility_changed");
}
