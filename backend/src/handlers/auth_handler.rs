use axum::{extract::State, http::StatusCode};
use validator::Validate;

use crate::{
    AppState,
    auth::{jwt, password},
    errors::AppError,
    extractors::Json,
    models::user::{LoginRequest, RefreshRequest, RegisterRequest, TokenResponse},
    repositories::user_repository,
};

/// `POST /auth/register`
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), AppError> {
    input.validate()?;

    let password_hash = password::hash(&input.password)?;
    let user =
        user_repository::create(&state.db, &input.email, &password_hash, &input.display_name)
            .await?;

    let token_response = create_token_pair(&state, user.id)?;
    Ok((StatusCode::CREATED, Json(token_response)))
}

/// `POST /auth/login`
///
/// Both "email not found" and "wrong password" return `401` — intentionally
/// indistinct to prevent username enumeration.
pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    input.validate()?;

    let user = user_repository::find_by_email(&state.db, &input.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify(&input.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token_response = create_token_pair(&state, user.id)?;
    Ok(Json(token_response))
}

/// `POST /auth/refresh`
///
/// Validates the refresh token signature and issues a new token pair.
/// Invalidation is handled externally (Redis).
pub async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let claims = jwt::validate_token(&input.refresh_token, &state.config.jwt_secret)?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized);
    }

    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    let token_response = create_token_pair(&state, user_id)?;
    Ok(Json(token_response))
}

fn create_token_pair(state: &AppState, user_id: uuid::Uuid) -> Result<TokenResponse, AppError> {
    let access_token = jwt::create_access_token(
        user_id,
        &state.config.jwt_secret,
        state.config.jwt_access_expiry_secs,
    )?;
    let refresh_token = jwt::create_refresh_token(
        user_id,
        &state.config.jwt_secret,
        state.config.jwt_refresh_expiry_secs,
    )?;
    Ok(TokenResponse {
        access_token,
        refresh_token,
    })
}
