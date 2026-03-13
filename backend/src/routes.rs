use std::time::Duration;

use axum::{
    Router,
    http::{HeaderValue, Request, Response},
    routing::{delete, get, post},
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::Span;

use crate::{
    AppState,
    handlers::{
        auth_handler, comment_handler, health_handler, issue_handler, project_handler, user_handler,
    },
};

/// Build the full application router with all routes and middleware.
pub fn build(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin: &HeaderValue, _| {
            origin.as_bytes().starts_with(b"http://localhost:")
                || origin.as_bytes() == b"http://localhost"
        }))
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let trace = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            tracing::info_span!(
                "http_request",
                request_id = %uuid::Uuid::new_v4(),
                method = %req.method(),
                uri = %req.uri().path(), // omit query string if it has sensitive params
            )
        })
        .on_failure(()) // disable failure event logs
        .on_request(()) // disable default request event logs
        .on_response(|res: &Response<_>, latency: Duration, _span: &Span| {
            tracing::info!(
                status = res.status().as_u16(),
                latency_ms = latency.as_millis(),
            )
        });

    Router::new()
        .nest("/api", api_routes())
        .with_state(state)
        .layer(cors)
        .layer(trace)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health_routes())
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/projects", project_routes())
        .nest("/issues", issue_routes())
        .nest("/comments", comment_routes())
}

fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/live", get(health_handler::get_liveness))
        .route("/ready", get(health_handler::get_readiness))
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

fn project_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(project_handler::list_projects).post(project_handler::create_project),
        )
        .route(
            "/{project_id}",
            get(project_handler::get_project)
                .patch(project_handler::update_project)
                .delete(project_handler::delete_project),
        )
        .route(
            "/{project_id}/members",
            get(project_handler::list_members).post(project_handler::add_member),
        )
        .route(
            "/{project_id}/issues",
            get(issue_handler::list_issues).post(issue_handler::create_issue),
        )
}

fn issue_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/{issue_id}",
            get(issue_handler::get_issue)
                .patch(issue_handler::update_issue)
                .delete(issue_handler::delete_issue),
        )
        .route(
            "/{issue_id}/comments",
            get(comment_handler::list_comments).post(comment_handler::create_comment),
        )
}

fn comment_routes() -> Router<AppState> {
    Router::new().route("/{comment_id}", delete(comment_handler::delete_comment))
}
