use std::env;

/// Typed application configuration loaded from environment variables.
///
/// Required variables (`DATABASE_URL`, `JWT_SECRET`) cause a panic at startup
/// with a clear message if missing — fail-fast is intentional here.
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub jwt_secret: String,
    pub jwt_access_expiry_secs: i64,
    pub jwt_refresh_expiry_secs: i64,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: required("DATABASE_URL"),
            database_max_connections: optional("DATABASE_MAX_CONNECTIONS", 20),
            jwt_secret: required("JWT_SECRET"),
            jwt_access_expiry_secs: optional("JWT_ACCESS_EXPIRY_SECS", 300),
            jwt_refresh_expiry_secs: optional("JWT_REFRESH_EXPIRY_SECS", 604_800), // 7 days
            server_host: optional("SERVER_HOST", "0.0.0.0".to_string()),
            server_port: optional("SERVER_PORT", 8080),
            log_level: optional("LOG_LEVEL", "info".to_string()),
        }
    }
}

fn required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("missing required environment variable: {key}"))
}

fn optional<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
