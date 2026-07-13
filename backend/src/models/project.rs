use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::PatchField;
use crate::validation::{ValidationErrors, Validator, rules};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,

    /// URL-safe slug that uniquely identifies the project (e.g., "my-project").
    /// Allowed characters: lowercase letters, digits, and hyphens.
    /// Must start and end with a letter or digit.
    pub identifier: String,

    pub description: String,
    pub is_public: bool,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            identifier: String::new(),
            description: String::new(),
            is_public: false,
            owner_id: Uuid::nil(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Project {
    pub fn create(input: ProjectInput, owner_id: Uuid) -> Result<Self, ValidationErrors> {
        input.validate()?;
        let mut project = Project::default();
        project.owner_id = owner_id;
        let project = project.apply(input);
        project.validate()?;
        Ok(project)
    }

    pub fn update(self, input: ProjectInput) -> Result<Self, ValidationErrors> {
        input.validate()?;
        let mut project = self.apply(input);
        project.updated_at = Utc::now();
        project.validate()?;
        Ok(project)
    }

    pub fn apply(mut self, input: ProjectInput) -> Self {
        if let PatchField::Value(v) = input.name {
            self.name = v;
        }
        if let PatchField::Value(v) = input.identifier {
            self.identifier = v;
        }
        if let PatchField::Value(v) = input.description {
            self.description = v;
        }
        if let PatchField::Value(v) = input.is_public {
            self.is_public = v;
        }
        self
    }

    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut v = Validator::new();
        v.field("name", &self.name)
            .check(rules::not_empty)
            .check(rules::max_len(100));
        v.field("identifier", &self.identifier)
            .check(rules::between_len(3, 50))
            .check(rules::slug);
        v.field("description", &self.description)
            .check(rules::max_len(2_000));
        v.finish()
    }

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

/// Request body for `POST /api/projects` and `PATCH /api/projects/:project_id`.
///
/// All fields optional. Omitted fields are left unchanged.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct ProjectInput {
    pub name: PatchField<String>,
    pub identifier: PatchField<String>,
    pub description: PatchField<String>,
    pub is_public: PatchField<bool>,
}

impl ProjectInput {
    /// Validate fields that are present in this request.
    ///
    /// Absent fields are skipped.
    /// Domain-level post-merge validation runs separately in [`Project::validate`].
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        let mut v = Validator::new();
        v.field("name", &self.name)
            .check(rules::not_empty)
            .check(rules::max_len(100));
        v.field("identifier", &self.identifier)
            .check(rules::between_len(3, 50))
            .check(rules::slug);
        v.field("description", &self.description)
            .check(rules::max_len(2_000));
        v.finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "text", rename_all = "lowercase")]
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

/// Request body for `POST /api/projects/:project_id/members`.
#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub email: String,
    pub role: MemberRole,
}
