use std::sync::Arc;

use sqlx::PgPool;
use rust_chat_server::auth::jwt::JwtManager;
use rust_chat_server::auth::session::SessionManager;
use rust_chat_server::config::AppConfig;
use rust_chat_server::state::AppState;
use rust_chat_server::repositories::user_repository::UserRepository;
use rust_chat_server::repositories::space_repository::SpaceRepository;
use rust_chat_server::repositories::channel_repository::ChannelRepository;
use rust_chat_server::repositories::invite_repository::InviteRepository;
use rust_chat_server::repositories::message_repository::MessageRepository;
use rust_chat_server::repositories::file_repository::FileRepository;
use rust_chat_server::services::auth_service::AuthService;
use rust_chat_server::services::file_service::FileService;
use rust_chat_server::services::space_service::SpaceService;
use rust_chat_server::services::channel_service::ChannelService;
use rust_chat_server::services::invite_service::InviteService;
use rust_chat_server::services::message_service::MessageService;
use rust_chat_server::services::presence_service::PresenceService;
use rust_chat_server::services::typing_service::TypingService;
use rust_chat_server::permissions::PermissionService;
use rust_chat_server::storage::provider::create_storage_provider;

pub async fn setup_test_app() -> (axum::Router, PgPool) {
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

    let config = AppConfig {
        server: rust_chat_server::config::ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 0,
        },
        database: rust_chat_server::config::DatabaseConfig {
            url: database_url.clone(),
        },
        redis: rust_chat_server::config::RedisConfig {
            url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string()),
        },
        auth: rust_chat_server::config::AuthConfig {
            session_secret: "test-session-secret".to_string(),
            password_pepper: "test-pepper".to_string(),
            jwt_secret: "test-jwt-secret-at-least-32-bytes-long!!".to_string(),
            jwt_access_ttl_seconds: 900,
            jwt_refresh_ttl_seconds: 2592000,
        },
        storage: rust_chat_server::config::StorageConfig {
            provider: "local".to_string(),
            local_dir: Some("/tmp/test-storage".to_string()),
        },
    };

    let jwt_manager = JwtManager::new(
        &config.auth.jwt_secret,
        config.auth.jwt_access_ttl_seconds,
        config.auth.jwt_refresh_ttl_seconds,
    );

    let redis_client = redis::Client::open(config.redis.url.as_str())
        .expect("Failed to create Redis client");
    let redis_conn = redis_client
        .get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    let user_repo = Arc::new(UserRepository::new(pool.clone()));
    let space_repo = Arc::new(SpaceRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let invite_repo = Arc::new(InviteRepository::new(pool.clone()));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let session_manager = Arc::new(SessionManager::new(redis_client, pool.clone()));

    let auth_service = Arc::new(AuthService::new(
        user_repo,
        session_manager,
        jwt_manager.clone(),
        config.clone(),
    ));

    let file_repo = FileRepository::new(pool.clone());
    let permission_service = PermissionService::new(pool.clone());
    let storage_provider = create_storage_provider("local", "./test-uploads", "http://localhost:8080");
    let file_service = FileService::new(file_repo, storage_provider, permission_service, 10485760);

    let state = AppState {
        db: pool.clone(),
        redis: redis_conn.clone(),
        config,
        jwt_manager,
        auth_service,
        file_service,
        space_service: SpaceService::new(space_repo),
        channel_service: ChannelService::new(channel_repo),
        invite_service: InviteService::new(invite_repo),
        message_service: MessageService::new(message_repo),
        presence_service: PresenceService::new(redis_conn.clone()),
        typing_service: TypingService::new(redis_conn),
        realtime_hub: Arc::new(rust_chat_server::realtime::hub::RealtimeHub::default()),
    };

    let app = axum::Router::new()
        .merge(rust_chat_server::handlers::auth::router())
        .with_state(state);

    (app, pool)
}
