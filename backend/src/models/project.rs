use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub identifier: String,
    pub description: String,
    pub is_public: bool,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request body for `POST /api/projects`.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(
        min = 1,
        max = 200,
        message = "Project name must be between 1 and 200 characters."
    ))]
    pub name: String,

    /// URL-safe slug that uniquely identifies the project (e.g., "my-project").
    /// Allowed characters: lowercase letters, digits, and hyphens.
    /// Must start and end with a letter or digit.
    #[validate(
        length(
            min = 1,
            max = 100,
            message = "Identifier must be between 1 and 100 characters."
        ),
        custom(function = "validate_identifier")
    )]
    pub identifier: String,

    #[validate(length(
        max = 10000,
        message = "Description must be at most 10,000 characters."
    ))]
    #[serde(default)]
    pub description: String,

    #[serde(default = "default_true")]
    pub is_public: bool,
}

/// Request body for `PATCH /api/projects/:project_id`.
///
/// All fields optional. Omitted fields are left unchanged.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProjectRequest {
    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(length(
        min = 1,
        max = 200,
        message = "Project name must be between 1 and 200 characters."
    ))]
    pub name: Option<String>,

    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(length(
        max = 10000,
        message = "Description must be at most 10,000 characters."
    ))]
    pub description: Option<String>,

    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    pub is_public: Option<bool>,
}

fn validate_identifier(value: &str) -> Result<(), validator::ValidationError> {
    let valid = !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !value.starts_with('-')
        && !value.ends_with('-');

    if valid {
        Ok(())
    } else {
        let mut err = validator::ValidationError::new("invalid_identifier");
        err.message = Some(
            "Identifier may only contain lowercase letters, digits, and hyphens, \
             and must start and end with a letter or digit."
                .into(),
        );
        Err(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum MemberRole {
    Admin,
    Member,
    Viewer,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProjectMember {
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub joined_at: DateTime<Utc>,
}

impl Project {
    /// Can the user see this project?
    /// Public projects are visible to everyone; private ones require ownership or membership.
    pub fn can_view(&self, user_id: Uuid, member_role: Option<MemberRole>) -> bool {
        self.is_public || self.owner_id == user_id || member_role.is_some()
    }

    /// Can the user comment on issues in this project?
    /// Requires explicit membership (any role) or ownership.
    pub fn can_comment(&self, user_id: Uuid, member_role: Option<MemberRole>) -> bool {
        self.owner_id == user_id || member_role.is_some()
    }

    /// Can the user create/update issues?
    /// Requires ownership or a write-capable role (admin, member — NOT viewer).
    pub fn can_write_issues(&self, user_id: Uuid, member_role: Option<MemberRole>) -> bool {
        if self.owner_id == user_id {
            return true;
        }
        matches!(member_role, Some(MemberRole::Admin | MemberRole::Member))
    }

    /// Can the user modify project settings?
    /// Owner or admin members only.
    pub fn can_admin(&self, user_id: Uuid, member_role: Option<MemberRole>) -> bool {
        self.owner_id == user_id || member_role == Some(MemberRole::Admin)
    }
}

/// Request body for `POST /api/projects/:project_id/members`.
#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
    pub role: MemberRole,
}

fn default_true() -> bool {
    true
}
