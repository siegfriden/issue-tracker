pub mod auth;
pub mod config;
pub mod errors;
pub mod extract;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;

use config::Config;
use sqlx::PgPool;

/// Shared application state injected into every handler via Axum's `State` extractor.
///
/// `PgPool` is internally reference-counted — cloning it is cheap and shares the
/// underlying pool. `Config` holds only primitive types, so it's also cheap to clone.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Config,
}
