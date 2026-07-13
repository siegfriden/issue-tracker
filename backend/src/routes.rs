use std::time::Duration;

use axum::{
    Json, Router,
    body::to_bytes,
    http::{
        HeaderValue, Method, Request, StatusCode,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::map_response,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

use crate::{
    AppState,
    errors::ErrorResponse,
    handlers::{auth_handler, health_handler, project_handler, user_handler},
};

/// Build the full application router with all routes and middleware.
pub fn build(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin: &HeaderValue, _| {
            origin.as_bytes().starts_with(b"http://localhost:")
                || origin.as_bytes() == b"http://localhost"
                || origin.as_bytes().starts_with(b"http://127.0.0.1:")
                || origin.as_bytes() == b"http://127.0.0.1"
        }))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

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
        .on_response(|res: &Response, latency: Duration, _span: &tracing::Span| {
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
        .layer(map_response(format_extractor_errors))
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health_routes())
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/projects", project_routes())
    //  .nest("/issues", issue_routes())
    //  .nest("/comments", comment_routes())
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
        .route("/logout", post(auth_handler::logout))
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
    //  .route(
    //      "/{project_id}/issues",
    //      get(issue_handler::list_issues).post(issue_handler::create_issue),
    //  )
}

// fn issue_routes() -> Router<AppState> {
//     Router::new()
//         .route(
//             "/{issue_id}",
//             get(issue_handler::get_issue)
//                 .patch(issue_handler::update_issue)
//                 .delete(issue_handler::delete_issue),
//         )
//         .route(
//             "/{issue_id}/comments",
//             get(comment_handler::list_comments).post(comment_handler::create_comment),
//         )
// }

// fn comment_routes() -> Router<AppState> {
//     Router::new().route("/{comment_id}", delete(comment_handler::delete_comment))
// }

/// Converts plain-text extractor rejections into the app's standard JSON error shape.
///
/// Axum's built-in extractors (`Json`, `Path`, `Query`) return `text/plain` bodies when
/// they fail (e.g. an invalid UUID in a path segment, or a malformed JSON body). Without
/// this middleware those responses would bypass [`AppError::into_response`] and reach the
/// client as plain text, breaking the `{ "message": "...", "data": ... }` contract.
///
/// The middleware is intentionally narrow: it only rewraps responses that are **both**
/// a 4xx client error **and** have a `text/plain` content-type, leaving all other
/// responses (including `AppError`-produced JSON and successful responses) untouched.
async fn format_extractor_errors(res: Response) -> Response {
    let status = res.status();
    let is_plain_text = res
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|val| val.to_str().ok())
        .is_some_and(|val| val.starts_with("text/plain"));

    if !status.is_client_error() || !is_plain_text {
        return res;
    }

    let Ok(body_bytes) = to_bytes(res.into_body(), usize::MAX).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let body_text = String::from_utf8_lossy(&body_bytes).into_owned();

    (
        status,
        Json(ErrorResponse {
            message: "Invalid request parameters.".to_string(),
            data: Some(serde_json::Value::String(body_text)),
        }),
    )
        .into_response()
}
