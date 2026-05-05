use std::env;
use std::str::FromStr;

pub fn init() {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let log_format = env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    let filter = tracing_subscriber::EnvFilter::from_str(&log_level).expect("Invalid RUST_LOG");

    match log_format.as_str() {
        "json" => {
            tracing_subscriber::fmt()
                .with_env_filter(filter.clone())
                .json()
                .init();
        }
        _ => {
            tracing_subscriber::fmt()
                .with_env_filter(filter.clone())
                .pretty()
                .init();
        }
    }
}
