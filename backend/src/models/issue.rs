use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, sqlx::FromRow, ToSchema)]
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
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IssueFilters {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assignee_id: Option<Uuid>,
}

/// Request body for `POST /api/projects/:project_id/issues`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateIssueRequest {
    #[validate(length(
        min = 1,
        max = 300,
        message = "Subject must be between 1 and 300 characters."
    ))]
    pub subject: String,

    #[validate(length(
        max = 10000,
        message = "Description must be at most 10,000 characters."
    ))]
    #[serde(default)]
    #[schema(default = "")]
    pub description: String,

    pub assignee_id: Option<Uuid>,

    #[validate(custom(function = "validate_issue_type"))]
    #[serde(default = "default_issue_type")]
    #[schema(default = "task")]
    pub issue_type: String,

    #[validate(custom(function = "validate_status"))]
    #[serde(default = "default_status")]
    #[schema(default = "open")]
    pub status: String,

    #[validate(custom(function = "validate_priority"))]
    #[serde(default = "default_priority")]
    #[schema(default = "normal")]
    pub priority: String,

    pub due_date: Option<NaiveDate>,
}

/// Request body for `PATCH /api/issues/:issue_id`.
///
/// All fields are optional — only provided fields are updated.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateIssueRequest {
    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(length(
        min = 1,
        max = 300,
        message = "Subject must be between 1 and 300 characters."
    ))]
    pub subject: Option<String>,

    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(length(
        max = 10000,
        message = "Description must be at most 10,000 characters."
    ))]
    pub description: Option<String>,

    #[serde(default, deserialize_with = "super::nullable::deserialize")]
    #[schema(value_type = Option<Uuid>, nullable)]
    pub assignee_id: Option<Option<Uuid>>,

    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(custom(function = "validate_issue_type"))]
    pub issue_type: Option<String>,

    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(custom(function = "validate_status"))]
    pub status: Option<String>,

    #[serde(default, deserialize_with = "super::non_nullable::deserialize")]
    #[validate(custom(function = "validate_priority"))]
    pub priority: Option<String>,

    #[serde(default, deserialize_with = "super::nullable::deserialize")]
    #[schema(value_type = Option<NaiveDate>, nullable)]
    pub due_date: Option<Option<NaiveDate>>,
}

fn validate_issue_type(value: &str) -> Result<(), ValidationError> {
    match value {
        "bug" | "feature" | "task" | "support" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_issue_type");
            err.message = Some("Issue type must be one of: bug, feature, task, support.".into());
            Err(err)
        }
    }
}

fn validate_status(value: &str) -> Result<(), ValidationError> {
    match value {
        "open" | "in_progress" | "resolved" | "closed" | "feedback" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_status");
            err.message = Some(
                "Status must be one of: open, in_progress, resolved, closed, feedback.".into(),
            );
            Err(err)
        }
    }
}

fn validate_priority(value: &str) -> Result<(), ValidationError> {
    match value {
        "low" | "normal" | "high" | "urgent" | "immediate" => Ok(()),
        _ => {
            let mut err = ValidationError::new("invalid_priority");
            err.message =
                Some("Priority must be one of: low, normal, high, urgent, immediate.".into());
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
