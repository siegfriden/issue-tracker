use super::PatchField;
use crate::validation::{ValidationErrors, Validator, rules};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl User {
    pub fn apply(mut self, input: &UserInput) -> Self {
        if let PatchField::Value(ref v) = input.display_name {
            self.display_name = v.clone();
        }
        self
    }

    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut v = Validator::new();
        v.field("email", &self.email).check(rules::email);
        v.field("display_name", &self.display_name)
            .check(rules::max_len(100));
        v.finish()
    }
}

/// Safe view of a user — everything except the password hash.
#[derive(Debug, Serialize)]
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
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}

impl RegisterRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut v = Validator::new();
        v.field("email", &self.email).check(rules::email);
        v.field("display_name", &self.display_name)
            .check(rules::max_len(100));
        v.field("password", &self.password)
            .check(rules::password_strength);
        v.finish()
    }
}

/// Request body for `POST /api/auth/login`.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut v = Validator::new();
        v.field("email", &self.email).check(rules::not_empty);
        v.field("password", &self.password).check(rules::not_empty);
        v.finish()
    }
}

/// Request body for `PATCH /api/users/me`.
///
/// All fields are optional — only provided fields are updated.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct UserInput {
    pub display_name: PatchField<String>,
    pub new_password: PatchField<String>,
}

impl UserInput {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut v = Validator::new();
        v.field("display_name", &self.display_name)
            .check(rules::max_len(100));
        v.field("new_password", &self.new_password)
            .check(rules::password_strength);
        v.finish()
    }
}
