use std::net::SocketAddr;

use issue_tracker_api::{AppState, config::Config, routes};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Load .env file if present (ignores errors when the file is absent)
    let _ = dotenvy::dotenv();

    let config = Config::from_env();

    // Structured logging
    // Release builds emit JSON; debug builds use human-readable output.
    let subscriber_builder =
        tracing_subscriber::fmt().with_env_filter(EnvFilter::new(&config.log_level));

    #[cfg(debug_assertions)]
    subscriber_builder.init();

    #[cfg(not(debug_assertions))]
    subscriber_builder.json().with_span_list(false).init();

    // Build PostgreSQL connection pool, panics on connection failure
    tracing::info!("connecting to database...");
    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|e| panic!("failed to connect to database: {e}"));

    tracing::info!("running migrations...");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .unwrap_or_else(|e| panic!("failed to run database migrations: {e}"));

    let addr_str: String = format!("{}:{}", config.server_host, config.server_port);
    let addr: SocketAddr = addr_str
        .parse()
        .unwrap_or_else(|_| panic!("invalid server address: {addr_str}"));

    let router = routes::build(AppState { db, config });

    tracing::info!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind to address: {e}"));

    axum::serve(listener, router)
        .await
        .unwrap_or_else(|e| panic!("server error: {e}"));
}
