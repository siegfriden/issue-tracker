use axum::{Json, extract::State};

use crate::{
    AppState,
    auth::{middleware::Auth, password},
    errors::AppError,
    models::{
        PatchField,
        user::{UserInput, UserResponse},
    },
    repositories::user_repository,
};

/// `GET /api/users/me`
///
/// Returns the authenticated user's profile.
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
pub async fn update_me(
    State(state): State<AppState>,
    auth: Auth,
    Json(input): Json<UserInput>,
) -> Result<Json<UserResponse>, AppError> {
    input.validate()?;

    let user = user_repository::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found.".to_string()))?;

    let mut user = user.apply(&input);

    if let PatchField::Value(ref pw) = input.new_password {
        user.password_hash = password::hash(pw)?;
    }

    user.validate()?;
    user.updated_at = chrono::Utc::now();

    user_repository::update(&state.db, &user).await?;
    Ok(Json(user.into()))
}
