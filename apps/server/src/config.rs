use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub session_secret: String,
    pub password_pepper: String,
    pub jwt_secret: String,
    pub jwt_access_ttl_seconds: i64,
    pub jwt_refresh_ttl_seconds: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub provider: String,
    pub local_dir: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .expect("SERVER_PORT must be a number"),
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set"),
            },
            redis: RedisConfig {
                url: std::env::var("REDIS_URL")
                    .expect("REDIS_URL must be set"),
            },
            auth: AuthConfig {
                session_secret: std::env::var("SESSION_SECRET")
                    .expect("SESSION_SECRET must be set"),
                password_pepper: std::env::var("PASSWORD_PEPPER")
                    .expect("PASSWORD_PEPPER must be set"),
                jwt_secret: std::env::var("JWT_SECRET")
                    .expect("JWT_SECRET must be set"),
                jwt_access_ttl_seconds: std::env::var("JWT_ACCESS_TTL_SECONDS")
                    .unwrap_or_else(|_| "900".to_string())
                    .parse()
                    .unwrap_or(900),
                jwt_refresh_ttl_seconds: std::env::var("JWT_REFRESH_TTL_SECONDS")
                    .unwrap_or_else(|_| "2592000".to_string())
                    .parse()
                    .unwrap_or(2592000),
            },
            storage: StorageConfig {
                provider: std::env::var("STORAGE_PROVIDER").unwrap_or_else(|_| "local".to_string()),
                local_dir: std::env::var("LOCAL_STORAGE_DIR").ok(),
            },
        }
    }
}