use std::collections::HashMap;

use axum::{
    Json,
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

/// Standard error response shape used in `AppError::into_response`.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// Human-readable error message.
    pub message: String,
    /// Additional error details (validation field errors, etc.) or null.
    pub data: Option<serde_json::Value>,
}

/// Application-wide error type. Every handler returns `Result<_, AppError>`.
///
/// Implements `IntoResponse` so Axum can convert it directly into an HTTP
/// response — no error-mapping boilerplate needed in handlers.
#[derive(Debug)]
pub enum AppError {
    /// 400 — malformed request that doesn't fit the validation framework.
    BadRequest(String),

    /// 400 — field-level validation failures from the `validator` crate.
    /// The inner map is `{ field: [messages] }` and is included in the
    /// response body under the `data` key.
    Validation(HashMap<String, Vec<String>>),

    /// 401 — missing, expired, or invalid credentials.
    Unauthorized,

    /// 403 — authenticated but not allowed to perform this action.
    Forbidden,

    /// 404 — resource does not exist (also used when existence must not be revealed).
    NotFound(String),

    /// 409 — uniqueness constraint violation (e.g. duplicate email).
    Conflict(String),

    /// 500 — unexpected server-side failure. The message is logged but never
    /// sent to the client.
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
        let (status, message, data) = match self {
            AppError::BadRequest(detail) => (
                StatusCode::BAD_REQUEST,
                "Invalid request parameters.".to_string(),
                Some(serde_json::json!(detail)),
            ),
            AppError::Validation(fields) => (
                StatusCode::BAD_REQUEST,
                "Missing or invalid fields. Please check your input and try again.".to_string(),
                Some(serde_json::json!(fields)),
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials. Please log in again.".to_string(),
                None,
            ),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Access denied.".to_string(), None),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg, None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg, None),
            AppError::Internal(msg) => {
                tracing::error!(error = %msg, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred. Please try again later.".to_string(),
                    None,
                )
            }
        };

        let body = serde_json::json!(ErrorResponse { message, data });
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

/// Converts Axum's [`JsonRejection`] into [`AppError::BadRequest`].
/// Used by the `#[derive(FromRequest)]` macro on [`crate::extractors::Json`].
impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        AppError::BadRequest(rejection.body_text())
    }
}

/// Converts Axum's [`PathRejection`] into [`AppError::BadRequest`].
/// Used by the `#[derive(FromRequestParts)]` macro on [`crate::extractors::Path`].
impl From<PathRejection> for AppError {
    fn from(rejection: PathRejection) -> Self {
        AppError::BadRequest(rejection.body_text())
    }
}

/// Converts Axum's [`QueryRejection`] into [`AppError::BadRequest`].
/// Used by the `#[derive(FromRequestParts)]` macro on [`crate::extractors::Query`].
impl From<QueryRejection> for AppError {
    fn from(rejection: QueryRejection) -> Self {
        AppError::BadRequest(rejection.body_text())
    }
}

/// Converts `sqlx::Error` into the appropriate `AppError` variant:
/// - `RowNotFound` → `NotFound`
/// - Everything else → `Internal`
///
/// Unique-violation (23505) conflicts are detected explicitly in handlers
/// before INSERT or UPDATE, so unexpected 23505s surface as 500 Internal.
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found.".to_string()),
            sqlx::Error::Database(db_err) => AppError::Internal(db_err.to_string()),
            other => AppError::Internal(other.to_string()),
        }
    }
}
