use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use uuid::Uuid;

use crate::{
    AppState,
    auth::{cookie, jwt},
    errors::AppError,
};

/// Axum extractor that authenticates a request via the `access_token` httpOnly cookie.
///
/// Add it as a parameter to any handler that requires authentication:
///
/// ```rust
/// async fn my_handler(
///     State(state): State<AppState>,
///     auth: Auth,
/// ) -> Result<Json<...>, AppError> { ... }
/// ```
#[derive(Debug, Clone)]
pub struct Auth {
    pub user_id: Uuid,
}

impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let token = cookie::extract_cookie(parts, "access_token").ok_or(AppError::Unauthorized)?;

        let claims = jwt::validate_token(&token, &app_state.config.jwt_secret)?;

        if claims.token_type != "access" {
            return Err(AppError::Unauthorized);
        }

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

        Ok(Auth { user_id })
    }
}
