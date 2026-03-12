use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    auth::middleware::Auth,
    errors::AppError,
    models::{
        PaginatedResponse, PaginationParams,
        comment::{Comment, CreateCommentRequest},
    },
    repositories::{comment_repository, issue_repository, project_repository},
};

/// `GET /api/issues/:issue_id/comments`
pub async fn list_comments(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Comment>>, AppError> {
    let issue = issue_repository::find_by_id(&state.db, issue_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let project = project_repository::find_by_id(&state.db, issue.project_id)
        .await?
        .ok_or(AppError::NotFound)?;

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
pub async fn create_comment(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
    Json(input): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    input.validate()?;

    let issue = issue_repository::find_by_id(&state.db, issue_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let project = project_repository::find_by_id(&state.db, issue.project_id)
        .await?
        .ok_or(AppError::NotFound)?;

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
pub async fn delete_comment(
    State(state): State<AppState>,
    auth: Auth,
    Path(comment_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let comment = comment_repository::find_by_id(&state.db, comment_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if comment.author_id != auth.user_id {
        return Err(AppError::Forbidden);
    }

    comment_repository::delete(&state.db, comment_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
