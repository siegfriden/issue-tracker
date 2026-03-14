use axum::extract::State;
use validator::Validate;

use crate::{
    AppState,
    auth::{middleware::Auth, password},
    errors::AppError,
    extractors::Json,
    models::user::{UpdateUserRequest, UserResponse},
    repositories::user_repository,
};

/// `GET /api/users/me`
///
/// Returns the authenticated user's profile.
#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Users",
    summary = "Get current user",
    description = "Returns the authenticated user's profile.",
    responses(
        (status = 200, description = "User profile", body = UserResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn get_me(
    State(state): State<AppState>,
    auth: Auth,
) -> Result<Json<UserResponse>, AppError> {
    let user = user_repository::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found.".to_string()))?;

    Ok(Json(user.into()))
}

/// `PATCH /api/users/me`
///
/// Updates the authenticated user's profile. Only supplied fields are changed.
#[utoipa::path(
    patch,
    path = "/api/users/me",
    tag = "Users",
    summary = "Update current user",
    description = "Updates the authenticated user's display name and/or password. Only supplied fields are changed.",
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 400, description = "Validation error", body = crate::errors::ErrorResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn update_me(
    State(state): State<AppState>,
    auth: Auth,
    Json(input): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    input.validate()?;

    let new_hash = match &input.new_password {
        Some(pw) => Some(password::hash(pw)?),
        None => None,
    };

    let user = user_repository::update(
        &state.db,
        auth.user_id,
        input.display_name.as_deref(),
        new_hash.as_deref(),
    )
    .await?;

    Ok(Json(user.into()))
}
