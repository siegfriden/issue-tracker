use axum::{
    extract::State,
    http::{StatusCode, header::SET_COOKIE, request::Parts},
    response::AppendHeaders,
};
use validator::Validate;

use crate::{
    AppState,
    auth::{cookie, jwt, password},
    errors::AppError,
    extract::Json,
    models::user::{LoginRequest, RegisterRequest},
    repositories::user_repository,
};

/// `POST /auth/register`
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<
    (
        StatusCode,
        AppendHeaders<[(axum::http::HeaderName, axum::http::HeaderValue); 2]>,
    ),
    AppError,
> {
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

    Ok((StatusCode::CREATED, build_cookie_headers(&state, user.id)?))
}

/// `POST /auth/login`
///
/// Both "email not found" and "wrong password" return `401` — intentionally
/// indistinct to prevent username enumeration.
pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<AppendHeaders<[(axum::http::HeaderName, axum::http::HeaderValue); 2]>, AppError> {
    input.validate()?;

    let user = user_repository::find_by_email(&state.db, &input.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify(&input.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    build_cookie_headers(&state, user.id)
}

/// `POST /auth/refresh`
///
/// Reads the refresh token from the `refresh_token` cookie, validates it, and
/// issues a new token pair as httpOnly cookies.
pub async fn refresh(
    State(state): State<AppState>,
    parts: Parts,
) -> Result<AppendHeaders<[(axum::http::HeaderName, axum::http::HeaderValue); 2]>, AppError> {
    let refresh_token =
        cookie::extract_cookie(&parts, "refresh_token").ok_or(AppError::Unauthorized)?;

    let claims = jwt::validate_token(&refresh_token, &state.config.jwt_secret)?;

    if claims.token_type != "refresh" {
        return Err(AppError::Unauthorized);
    }

    let user_id = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;
    build_cookie_headers(&state, user_id)
}

/// `POST /auth/logout`
///
/// Clears the access and refresh token cookies.
pub async fn logout() -> AppendHeaders<[(axum::http::HeaderName, axum::http::HeaderValue); 2]> {
    AppendHeaders([
        (SET_COOKIE, cookie::clear_cookie("access_token", "/")),
        (
            SET_COOKIE,
            cookie::clear_cookie("refresh_token", "/api/auth/refresh"),
        ),
    ])
}

fn build_cookie_headers(
    state: &AppState,
    user_id: uuid::Uuid,
) -> Result<AppendHeaders<[(axum::http::HeaderName, axum::http::HeaderValue); 2]>, AppError> {
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
    Ok(AppendHeaders([
        (
            SET_COOKIE,
            cookie::build_cookie(
                "access_token",
                &access_token,
                state.config.jwt_access_expiry_secs,
                "/",
            ),
        ),
        (
            SET_COOKIE,
            cookie::build_cookie(
                "refresh_token",
                &refresh_token,
                state.config.jwt_refresh_expiry_secs,
                "/api/auth/refresh",
            ),
        ),
    ]))
}
