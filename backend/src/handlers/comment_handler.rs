use axum::{extract::State, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    auth::middleware::Auth,
    errors::AppError,
    extractors::{Json, Path, Query},
    models::{
        PaginatedResponse, PaginationParams,
        comment::{Comment, CreateCommentRequest},
    },
    repositories::{comment_repository, issue_repository, project_repository},
};

/// `GET /api/issues/:issue_id/comments`
#[utoipa::path(
    get,
    path = "/api/issues/{issue_id}/comments",
    tag = "Comments",
    summary = "List comments",
    description = "Lists comments for an issue. Requires visibility access to the parent project.",
    params(
        ("issue_id" = Uuid, Path, description = "Issue ID"),
        PaginationParams,
    ),
    responses(
        (status = 200, description = "Paginated list of comments", body = crate::models::paginated_schemas::PaginatedCommentResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::errors::ErrorResponse),
        (status = 404, description = "Issue not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn list_comments(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Comment>>, AppError> {
    let issue = issue_repository::find_by_id(&state.db, issue_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Issue not found.".to_string()))?;

    let project = project_repository::find_by_id(&state.db, issue.project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;
    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let (comments, total) =
        comment_repository::find_by_issue(&state.db, issue_id, &pagination).await?;
    Ok(Json(PaginatedResponse::new(comments, total, &pagination)))
}

/// `POST /api/issues/:issue_id/comments`
#[utoipa::path(
    post,
    path = "/api/issues/{issue_id}/comments",
    tag = "Comments",
    summary = "Create comment",
    description = "Creates a new comment on an issue. Requires membership in the parent project.",
    params(("issue_id" = Uuid, Path, description = "Issue ID")),
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment created", body = Comment),
        (status = 400, description = "Validation error", body = crate::errors::ErrorResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::errors::ErrorResponse),
        (status = 404, description = "Issue not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn create_comment(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
    Json(input): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    input.validate()?;

    let issue = issue_repository::find_by_id(&state.db, issue_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Issue not found.".to_string()))?;

    let project = project_repository::find_by_id(&state.db, issue.project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;
    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }
    if !project.can_comment(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let comment =
        comment_repository::create(&state.db, issue_id, auth.user_id, &input.body).await?;
    Ok((StatusCode::CREATED, Json(comment)))
}

/// `DELETE /api/comments/:comment_id`
///
/// Only the comment's author can delete a comment.
#[utoipa::path(
    delete,
    path = "/api/comments/{comment_id}",
    tag = "Comments",
    summary = "Delete comment",
    description = "Deletes a comment. Only the comment author can perform this action.",
    params(("comment_id" = Uuid, Path, description = "Comment ID")),
    responses(
        (status = 204, description = "Comment deleted"),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied (not author)", body = crate::errors::ErrorResponse),
        (status = 404, description = "Comment not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn delete_comment(
    State(state): State<AppState>,
    auth: Auth,
    Path(comment_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let comment = comment_repository::find_by_id(&state.db, comment_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Comment not found.".to_string()))?;

    if comment.author_id != auth.user_id {
        return Err(AppError::Forbidden);
    }

    comment_repository::delete(&state.db, comment_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
