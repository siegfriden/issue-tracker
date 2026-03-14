use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Full database row. `password_hash` is excluded from serialization — it
/// must never appear in an API response.
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Safe view of a user — everything except the password hash.
#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            display_name: u.display_name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}

/// Request body for `POST /api/auth/register`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email(message = "Please enter a valid email address."))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long."))]
    pub password: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Display name must be between 1 and 100 characters."
    ))]
    pub display_name: String,
}

/// Request body for `POST /api/auth/login`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "Please enter a valid email address."))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required."))]
    pub password: String,
}

/// Request body for `POST /api/auth/refresh`.
#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Response returned after login or refresh.
#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

/// Request body for `PATCH /api/users/me`.
///
/// All fields are optional — only provided fields are updated.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Display name must be between 1 and 100 characters."
    ))]
    pub display_name: Option<String>,

    #[validate(length(min = 8, message = "New password must be at least 8 characters long."))]
    pub new_password: Option<String>,
}
