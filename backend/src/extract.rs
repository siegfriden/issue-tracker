//! Newtype wrappers around Axum's built-in extractors that route rejection
//! errors through [`AppError`], keeping every error response in the
//! `{ "message": "...", "data": ... }` shape defined by [`ErrorResponse`].
//!
//! The derive macros delegate extraction to the underlying Axum type and
//! convert its rejection via the corresponding `From<*Rejection> for AppError`
//! impls in [`crate::errors`].
//!
//! [`ErrorResponse`]: crate::errors::ErrorResponse

use axum::{
    extract::{FromRequest, FromRequestParts},
    response::IntoResponse,
};
use serde::Serialize;

use crate::errors::AppError;

/// Wrapper around [`axum::Json`] that maps deserialization failures to
/// [`AppError::BadRequest`]. Also implements [`IntoResponse`] so it can
/// be used in both extractor and return position.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct Json<T>(pub T);

/// Wrapper around [`axum::extract::Path`] that maps path-parameter parse
/// failures (e.g. invalid UUID) to [`AppError::BadRequest`].
#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(AppError))]
pub struct Path<T>(pub T);

/// Wrapper around [`axum::extract::Query`] that maps query-string parse
/// failures to [`AppError::BadRequest`].
#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(AppError))]
pub struct Query<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
