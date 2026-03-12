use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    AppState,
    handlers::{auth_handler, health_handler, user_handler},
};

/// Build the full application router with all routes and middleware.
pub fn build(state: AppState) -> Router {
    Router::new().nest("/api", api_routes()).with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_handler::get_health))
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .route("/refresh", post(auth_handler::refresh))
}

fn user_routes() -> Router<AppState> {
    Router::new().route(
        "/me",
        get(user_handler::get_me).patch(user_handler::update_me),
    )
}
