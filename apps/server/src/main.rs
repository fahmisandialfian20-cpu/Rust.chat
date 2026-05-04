use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use axum::http::HeaderName;

use rust_chat_server::config::AppConfig;
use rust_chat_server::state::AppState;
use rust_chat_server::auth::jwt::JwtManager;
use rust_chat_server::auth::session::SessionManager;
use rust_chat_server::repositories::user_repository::UserRepository;
use rust_chat_server::repositories::space_repository::SpaceRepository;
use rust_chat_server::repositories::channel_repository::ChannelRepository;
use rust_chat_server::repositories::invite_repository::InviteRepository;
use rust_chat_server::repositories::message_repository::MessageRepository;
use rust_chat_server::repositories::file_repository::FileRepository;
use rust_chat_server::services::auth_service::AuthService;
use rust_chat_server::services::file_service::FileService;
use rust_chat_server::permissions::PermissionService;
use rust_chat_server::storage::provider::create_storage_provider;
use rust_chat_server::services::space_service::SpaceService;
use rust_chat_server::services::channel_service::ChannelService;
use rust_chat_server::services::invite_service::InviteService;
use rust_chat_server::services::message_service::MessageService;
use rust_chat_server::services::presence_service::PresenceService;
use rust_chat_server::services::typing_service::TypingService;

#[tokio::main]
async fn main() {
    let _ = rust_chat_server::telemetry::init();

    let config = AppConfig::from_env();

    let db = sqlx::PgPool::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    let redis_client = redis::Client::open(config.redis.url.as_str())
        .expect("Failed to create Redis client");

    let redis = redis_client
        .get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    let jwt_manager = JwtManager::new(
        &config.auth.jwt_secret,
        config.auth.jwt_access_ttl_seconds,
        config.auth.jwt_refresh_ttl_seconds,
    );

    let user_repo = Arc::new(UserRepository::new(db.clone()));
    let space_repo = Arc::new(SpaceRepository::new(db.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(db.clone()));
    let invite_repo = Arc::new(InviteRepository::new(db.clone()));
    let message_repo = Arc::new(MessageRepository::new(db.clone()));
    let file_repo = FileRepository::new(db.clone());

    let session_manager = Arc::new(SessionManager::new(redis_client, db.clone()));

    let auth_service = Arc::new(AuthService::new(
        user_repo.clone(),
        session_manager.clone(),
        jwt_manager.clone(),
        config.clone(),
    ));

    let space_service = SpaceService::new(space_repo);
    let channel_service = ChannelService::new(channel_repo);
    let invite_service = InviteService::new(invite_repo);
    let message_service = MessageService::new(message_repo);
    let presence_service = PresenceService::new(redis.clone());
    let typing_service = TypingService::new(redis.clone());
    let realtime_hub = Arc::new(rust_chat_server::realtime::hub::RealtimeHub::default());

    let permission_service = PermissionService::new(db.clone());
    let storage_provider = create_storage_provider(
        &config.storage.provider,
        config.storage.local_dir.as_deref().unwrap_or("./uploads"),
        &config.server.host,
    );
    let max_upload_bytes = 10485760;
    let file_service = FileService::new(
        file_repo,
        storage_provider,
        permission_service.clone(),
        max_upload_bytes,
    );

    let state = AppState {
        db: db.clone(),
        redis,
        config: config.clone(),
        jwt_manager,
        auth_service,
        file_service,
        space_service,
        channel_service,
        invite_service,
        message_service,
        presence_service,
        typing_service,
        realtime_hub,
    };

    let sensitive_headers: Vec<HeaderName> = vec![
        "authorization".try_into().unwrap(),
        "cookie".try_into().unwrap(),
    ];

    let app = axum::Router::new()
        .merge(rust_chat_server::routes::health::router())
        .merge(rust_chat_server::handlers::auth::router())
        .route("/api/v1/ws", axum::routing::any(rust_chat_server::realtime::gateway::ws_upgrade))
        .nest("/api/v1", rust_chat_server::handlers::spaces::router())
        .nest("/api/v1", rust_chat_server::handlers::channels::router())
        .nest("/api/v1", rust_chat_server::handlers::invites::router())
        .nest("/api/v1", rust_chat_server::handlers::messages::router())
        .nest("/api/v1", rust_chat_server::handlers::files::router())
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new())
                .layer(SetSensitiveRequestHeadersLayer::new(sensitive_headers))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(state);

    let addr = SocketAddr::new(
        config.server.host.parse().unwrap(),
        config.server.port,
    );

    println!("Server running on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
