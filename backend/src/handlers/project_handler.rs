use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    AppState,
    auth::middleware::Auth,
    errors::AppError,
    models::{
        PaginatedResponse, PaginationParams,
        project::{AddMemberRequest, MemberRole, Project, ProjectInput, ProjectMember},
    },
    repositories::{project_repository, user_repository},
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
    Json(input): Json<ProjectInput>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    let project = Project::create(input, auth.user_id).map_err(AppError::Validation)?;

    if project_repository::exists_by_identifier(&state.db, &project.identifier).await? {
        return Err(AppError::Conflict(
            "A project with that identifier already exists.".to_string(),
        ));
    }

    project_repository::create(&state.db, &project).await?;
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
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

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
    Json(input): Json<ProjectInput>,
) -> Result<Json<Project>, AppError> {
    input.validate()?;

    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;

    if !project.can_admin(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let project = project.update(input).map_err(AppError::Validation)?;
    project_repository::update(&state.db, &project).await?;
    Ok(Json(project))
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
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

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
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;

    if !project.can_view(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    let (members, total) =
        project_repository::find_members(&state.db, project.id, &pagination).await?;
    Ok(Json(PaginatedResponse::new(members, total, &pagination)))
}

/// `POST /api/projects/:project_id/members`
///
/// Adds a user to a project with the given role. Requires admin access.
pub async fn add_member(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Json(input): Json<AddMemberRequest>,
) -> Result<StatusCode, AppError> {
    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let requester_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;
    if !project.can_admin(auth.user_id, requester_role) {
        return Err(AppError::Forbidden);
    }

    let user_to_add = user_repository::find_by_email(&state.db, &input.email)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found with that email.".to_string()))?;

    if project_repository::find_member_role(&state.db, project.id, user_to_add.id)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict(
            "User is already a member of this project.".to_string(),
        ));
    }

    project_repository::add_member(&state.db, project.id, user_to_add.id, input.role).await?;
    Ok(StatusCode::NO_CONTENT)
}
