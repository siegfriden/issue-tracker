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
        project::{CreateProjectRequest, MemberRole, Project, ProjectMember, UpdateProjectRequest},
    },
    repositories::project_repository,
};

/// `GET /api/projects`
///
/// Returns all projects the authenticated user can see:
/// public projects plus projects they own.
pub async fn list_projects(
    State(state): State<AppState>,
    auth: Auth,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Project>>, AppError> {
    let (projects, total) =
        project_repository::find_all_accessible(&state.db, auth.user_id, &pagination).await?;
    Ok(Json(PaginatedResponse::new(projects, total, &pagination)))
}

/// `POST /api/projects`
pub async fn create_project(
    State(state): State<AppState>,
    auth: Auth,
    Json(input): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    input.validate()?;

    let project = project_repository::create(&state.db, &input, auth.user_id).await?;
    project_repository::add_member(&state.db, project.id, auth.user_id, MemberRole::Admin).await?;
    Ok((StatusCode::CREATED, Json(project)))
}

/// `GET /api/projects/:project_id`
pub async fn get_project(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
) -> Result<Json<Project>, AppError> {
    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;

    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    Ok(Json(project))
}

/// `PATCH /api/projects/:project_id`
///
/// Owner or members with `admin` role can update a project.
pub async fn update_project(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Json(input): Json<UpdateProjectRequest>,
) -> Result<Json<Project>, AppError> {
    input.validate()?;

    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;

    if !project.can_admin(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let updated = project_repository::update(&state.db, project_id, &input).await?;
    Ok(Json(updated))
}

/// `DELETE /api/projects/:project_id`
///
/// Only the project owner can delete a project.
pub async fn delete_project(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if project.owner_id != auth.user_id {
        return Err(AppError::Forbidden);
    }

    project_repository::delete(&state.db, project_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// `GET /api/projects/:project_id/members`
///
/// Returns the list of members for a project. Requires the same visibility
/// check as viewing the project itself.
pub async fn list_members(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<ProjectMember>>, AppError> {
    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;

    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let (members, total) =
        project_repository::find_members(&state.db, project.id, &pagination).await?;
    Ok(Json(PaginatedResponse::new(members, total, &pagination)))
}
