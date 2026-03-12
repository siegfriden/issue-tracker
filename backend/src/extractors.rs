use axum::{
    Json as AxumJson,
    extract::{FromRequest, FromRequestParts, Path as AxumPath, Query as AxumQuery, Request},
    http::request::Parts,
    response::IntoResponse,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::AppError;

/// Newtype wrapper around `axum::Json` that maps deserialization failures to
/// `AppError::BadRequest` instead of Axum's built-in `JsonRejection` response,
/// keeping all error responses in the `{"error": "..."}` shape.
pub struct Json<T>(pub T);

/// Newtype wrapper around `axum::extract::Path` that maps path parameter
/// parse failures (e.g. invalid UUID) to `AppError::BadRequest`.
pub struct Path<T>(pub T);

/// Newtype wrapper around `axum::extract::Query` that maps query string
/// parse failures to `AppError::BadRequest`.
pub struct Query<T>(pub T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let AxumJson(value) = AxumJson::<T>::from_request(req, state)
            .await
            .map_err(|rejection| AppError::BadRequest(rejection.body_text()))?;
        Ok(Json(value))
    }
}

impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AxumPath(value) = AxumPath::<T>::from_request_parts(parts, state)
            .await
            .map_err(|rejection| AppError::BadRequest(rejection.body_text()))?;
        Ok(Path(value))
    }
}

impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AxumQuery(value) = AxumQuery::<T>::from_request_parts(parts, state)
            .await
            .map_err(|rejection| AppError::BadRequest(rejection.body_text()))?;
        Ok(Query(value))
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        AxumJson(self.0).into_response()
    }
}
