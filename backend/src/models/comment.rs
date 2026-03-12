use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub issue_id: Uuid,
    pub author_id: Uuid,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request body for `POST /api/issues/:issue_id/comments`.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Comment must be between 1 and 10,000 characters."
    ))]
    pub body: String,
}
