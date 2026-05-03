use axum::Router;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod config;
mod error;
mod routes;
mod state;

use config::AppConfig;
use routes::health;
use state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();

    let db = sqlx::PgPool::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    let redis = redis::Client::open(config.redis.url.as_str())
        .expect("Failed to create Redis client")
        .get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    let state = AppState { db, redis };

    let app = Router::new()
        .merge(health::router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
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