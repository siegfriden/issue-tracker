use crate::AppState;
use axum::{extract::State, http::StatusCode};

/// `GET /health`
///
/// Basic health check that pings the database.
pub async fn get_health(State(state): State<AppState>) -> (StatusCode, &'static str) {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (StatusCode::OK, "OK"),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "database connection failed",
        ),
    }
}
