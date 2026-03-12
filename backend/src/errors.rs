use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use validator::ValidationErrors;

/// Application-wide error type. Every handler returns `Result<_, AppError>`.
///
/// Implements `IntoResponse` so Axum can convert it directly into an HTTP
/// response — no error-mapping boilerplate needed in handlers.
#[derive(Debug, Error)]
pub enum AppError {
    /// 400 — malformed request that doesn't fit the validation framework.
    #[error("{0}")]
    BadRequest(String),

    /// 400 — field-level validation failures from the `validator` crate.
    /// The inner map is `{ field: [messages] }` and is included in the
    /// response body under the `data` key.
    #[error("validation failed")]
    Validation(HashMap<String, Vec<String>>),

    /// 401 — missing, expired, or invalid credentials.
    #[error("unauthorized")]
    Unauthorized,

    /// 403 — authenticated but not allowed to perform this action.
    #[error("forbidden")]
    Forbidden,

    /// 404 — resource does not exist (also used when existence must not be revealed).
    #[error("resource not found")]
    NotFound,

    /// 409 — uniqueness constraint violation (e.g. duplicate email).
    #[error("{0}")]
    Conflict(String),

    /// 500 — unexpected server-side failure. The message is logged but never
    /// sent to the client.
    #[error("internal server error")]
    Internal(String),
}

/// Converts `AppError` into an HTTP response with a consistent JSON shape:
///
/// ```json
/// { "error": "<message>", "data": <field_errors | null> }
/// ```
///
/// `Internal` errors log the detail via `tracing` and return a generic
/// message to avoid leaking implementation details.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = self.to_string();
        let (status, data) = match &self {
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, None),
            AppError::Validation(fields) => {
                (StatusCode::BAD_REQUEST, Some(serde_json::json!(fields)))
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, None),
            AppError::Forbidden => (StatusCode::FORBIDDEN, None),
            AppError::NotFound => (StatusCode::NOT_FOUND, None),
            AppError::Conflict(_) => (StatusCode::CONFLICT, None),
            AppError::Internal(msg) => {
                tracing::error!(error = %msg, "internal server error");
                (StatusCode::INTERNAL_SERVER_ERROR, None)
            }
        };

        let body = serde_json::json!({ "error": message, "data": data });
        (status, Json(body)).into_response()
    }
}

/// Converts `validator::ValidationErrors` into `AppError::Validation`,
/// flattening each field's errors into a list of human-readable strings.
impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let data = errors
            .field_errors()
            .iter()
            .map(|(field, errs)| {
                let messages = errs
                    .iter()
                    .map(|e| {
                        e.message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("invalid value for '{}'", e.code))
                    })
                    .collect();
                (field.to_string(), messages)
            })
            .collect();
        AppError::Validation(data)
    }
}

/// Converts `sqlx::Error` into the appropriate `AppError` variant:
/// - `RowNotFound` → `NotFound`
/// - PostgreSQL error code `23505` (unique violation) → `Conflict`
/// - Everything else → `Internal`
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound,
            sqlx::Error::Database(db_err) => {
                // PostgreSQL unique violation
                if db_err.code().as_deref() == Some("23505") {
                    return AppError::Conflict(
                        "a record with that value already exists".to_string(),
                    );
                }
                AppError::Internal(db_err.to_string())
            }
            other => AppError::Internal(other.to_string()),
        }
    }
}
