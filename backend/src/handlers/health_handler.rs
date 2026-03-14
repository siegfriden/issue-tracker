use crate::AppState;
use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ReadinessResponse {
    server: &'static str,
    database: &'static str,
    // redis: &'static str,
}

/// `GET /health/live`
///
/// Liveness probe — confirms the process is running. No external dependencies checked.
#[utoipa::path(
    get,
    path = "/api/health/live",
    tag = "Health",
    summary = "Liveness probe",
    description = "Confirms the process is running. No external dependencies checked.",
    responses(
        (status = 200, description = "Service is alive"),
    )
)]
pub async fn get_liveness() -> StatusCode {
    StatusCode::OK
}

/// `GET /health/ready`
///
/// Readiness probe — confirms the service can handle requests by pinging the database.
#[utoipa::path(
    get,
    path = "/api/health/ready",
    tag = "Health",
    summary = "Readiness probe",
    description = "Confirms the service can handle requests by pinging the database.",
    responses(
        (status = 200, description = "Service is ready", body = ReadinessResponse),
        (status = 503, description = "Service is not ready", body = ReadinessResponse),
    )
)]
pub async fn get_readiness(State(state): State<AppState>) -> (StatusCode, Json<ReadinessResponse>) {
    let mut status = StatusCode::OK;
    let mut database = "OK";

    if sqlx::query("SELECT 1").execute(&state.db).await.is_err() {
        status = StatusCode::SERVICE_UNAVAILABLE;
        database = "error";
    }

    (
        status,
        Json(ReadinessResponse {
            server: "OK",
            database,
            // redis: "OK",
        }),
    )
}
