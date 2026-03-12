use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Issue {
    pub id: Uuid,
    pub project_id: Uuid,
    pub author_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub subject: String,
    pub description: String,
    pub issue_type: String,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Optional filters for `GET /api/projects/:project_id/issues`.
#[derive(Debug, Deserialize)]
pub struct IssueFilters {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assignee_id: Option<Uuid>,
}

/// Request body for `POST /api/projects/:project_id/issues`.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateIssueRequest {
    #[validate(length(min = 1, max = 300, message = "must be between 1 and 300 characters"))]
    pub subject: String,

    #[validate(length(max = 10000, message = "must be at most 10000 characters"))]
    #[serde(default)]
    pub description: String,

    pub assignee_id: Option<Uuid>,

    #[validate(custom(function = "validate_issue_type"))]
    #[serde(default = "default_issue_type")]
    pub issue_type: String,

    #[validate(custom(function = "validate_status"))]
    #[serde(default = "default_status")]
    pub status: String,

    #[validate(custom(function = "validate_priority"))]
    #[serde(default = "default_priority")]
    pub priority: String,

    pub due_date: Option<NaiveDate>,
}

/// Request body for `PATCH /api/issues/:issue_id`.
///
/// All fields are optional — only provided fields are updated.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateIssueRequest {
    #[validate(length(min = 1, max = 300, message = "must be between 1 and 300 characters"))]
    pub subject: Option<String>,

    #[validate(length(max = 10000, message = "must be at most 10000 characters"))]
    pub description: Option<String>,

    #[serde(default, deserialize_with = "super::nullable::deserialize")]
    pub assignee_id: Option<Option<Uuid>>,

    #[validate(custom(function = "validate_issue_type"))]
    pub issue_type: Option<String>,

    #[validate(custom(function = "validate_status"))]
    pub status: Option<String>,

    #[validate(custom(function = "validate_priority"))]
    pub priority: Option<String>,

    #[serde(default, deserialize_with = "super::nullable::deserialize")]
    pub due_date: Option<Option<NaiveDate>>,
}

fn validate_issue_type(value: &str) -> Result<(), ValidationError> {
    match value {
        "bug" | "feature" | "task" | "support" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_issue_type");
            err.message = Some("must be one of: bug, feature, task, support".into());
            Err(err)
        }
    }
}

fn validate_status(value: &str) -> Result<(), ValidationError> {
    match value {
        "open" | "in_progress" | "resolved" | "closed" | "feedback" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_status");
            err.message =
                Some("must be one of: open, in_progress, resolved, closed, feedback".into());
            Err(err)
        }
    }
}

fn validate_priority(value: &str) -> Result<(), ValidationError> {
    match value {
        "low" | "normal" | "high" | "urgent" | "immediate" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_priority");
            err.message = Some("must be one of: low, normal, high, urgent, immediate".into());
            Err(err)
        }
    }
}

fn default_issue_type() -> String {
    "task".to_string()
}

fn default_status() -> String {
    "open".to_string()
}

fn default_priority() -> String {
    "normal".to_string()
}
