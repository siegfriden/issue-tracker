use axum::{Router, routing::get};

use crate::{AppState, handlers::health_handler};

/// Build the full application router with all routes and middleware.
pub fn build(state: AppState) -> Router {
    Router::new().nest("/api", api_routes()).with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_handler::get_health))
}
