use axum::{extract::State, http::StatusCode};
use validator::Validate;

use crate::{
    AppState,
    auth::{jwt, password},
    errors::AppError,
    extract::Json,
    models::user::{LoginRequest, RefreshRequest, RegisterRequest, TokenResponse},
    repositories::user_repository,
};

/// `POST /auth/register`
#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Auth",
    summary = "Register a new user",
    description = "Creates a new user account and returns an access/refresh token pair.",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = TokenResponse),
        (status = 400, description = "Validation error", body = crate::errors::ErrorResponse),
        (status = 409, description = "Email already taken", body = crate::errors::ErrorResponse),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), AppError> {
    input.validate()?;

    if user_repository::exists_by_email(&state.db, &input.email).await? {
        return Err(AppError::Conflict(
            "A user with that email already exists.".to_string(),
        ));
    }

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
#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    summary = "Log in",
    description = "Authenticates with email and password, returns an access/refresh token pair.",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 400, description = "Validation error", body = crate::errors::ErrorResponse),
        (status = 401, description = "Invalid credentials", body = crate::errors::ErrorResponse),
    )
)]
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
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    tag = "Auth",
    summary = "Refresh tokens",
    description = "Validates a refresh token and issues a new access/refresh token pair.",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Tokens refreshed", body = TokenResponse),
        (status = 401, description = "Invalid or expired refresh token", body = crate::errors::ErrorResponse),
    )
)]
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
