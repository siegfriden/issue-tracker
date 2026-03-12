use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

/// JWT payload embedded in both access and refresh tokens.
///
/// `token_type` distinguishes access tokens (short-lived, used for API calls)
/// from refresh tokens (long-lived, used only to obtain new access tokens).
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject — the user's UUID as a string.
    pub sub: String,
    /// Either "access" or "refresh".
    pub token_type: String,
    /// Expiry as a UNIX timestamp (seconds).
    pub exp: usize,
    /// Issued-at as a UNIX timestamp (seconds).
    pub iat: usize,
}

pub fn create_access_token(
    user_id: Uuid,
    secret: &str,
    expiry_secs: i64,
) -> Result<String, AppError> {
    create_token(user_id, "access", secret, expiry_secs)
}

pub fn create_refresh_token(
    user_id: Uuid,
    secret: &str,
    expiry_secs: i64,
) -> Result<String, AppError> {
    create_token(user_id, "refresh", secret, expiry_secs)
}

fn create_token(
    user_id: Uuid,
    token_type: &str,
    secret: &str,
    expiry_secs: i64,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        token_type: token_type.to_string(),
        exp: now + expiry_secs as usize,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("failed to create JWT: {e}")))
}

/// Validate a token string and return its claims.
///
/// Returns `AppError::Unauthorized` for any validation failure — expired,
/// malformed, wrong signature, etc.
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}
