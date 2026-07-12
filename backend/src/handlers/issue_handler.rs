use axum::{extract::{State, Path, Query}, http::StatusCode, Json};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    auth::middleware::Auth,
    errors::AppError,
    models::{
        PaginatedResponse, PaginationParams,
        issue::{CreateIssueRequest, Issue, IssueFilters, UpdateIssueRequest},
    },
    repositories::{issue_repository, project_repository},
};

/// `GET /api/projects/:project_id/issues`
///
/// Lists issues for a project with optional filters and pagination.
pub async fn list_issues(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Query(filters): Query<IssueFilters>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Issue>>, AppError> {
    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;
    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let (issues, total) =
        issue_repository::find_by_project(&state.db, project_id, &filters, &pagination).await?;

    Ok(Json(PaginatedResponse::new(issues, total, &pagination)))
}

/// `POST /api/projects/:project_id/issues`
pub async fn create_issue(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Json(input): Json<CreateIssueRequest>,
) -> Result<(StatusCode, Json<Issue>), AppError> {
    input.validate()?;

    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;
    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }
    if !project.can_write_issues(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let issue = issue_repository::create(&state.db, &input, project_id, auth.user_id).await?;
    Ok((StatusCode::CREATED, Json(issue)))
}

/// `GET /api/issues/:issue_id`
pub async fn get_issue(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
) -> Result<Json<Issue>, AppError> {
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

    Ok(Json(issue))
}

/// `PATCH /api/issues/:issue_id`
pub async fn update_issue(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
    Json(input): Json<UpdateIssueRequest>,
) -> Result<Json<Issue>, AppError> {
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
    if !project.can_write_issues(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let updated = issue_repository::update(&state.db, issue_id, &input).await?;
    Ok(Json(updated))
}

/// `DELETE /api/issues/:issue_id`
///
/// Only the issue author can delete an issue.
pub async fn delete_issue(
    State(state): State<AppState>,
    auth: Auth,
    Path(issue_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let issue = issue_repository::find_by_id(&state.db, issue_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Issue not found.".to_string()))?;

    if issue.author_id != auth.user_id {
        return Err(AppError::Forbidden);
    }

    issue_repository::delete(&state.db, issue_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
