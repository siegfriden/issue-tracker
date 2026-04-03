use axum::{extract::State, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use crate::{
    AppState,
    auth::middleware::Auth,
    errors::AppError,
    extract::{Json, Path, Query},
    models::{
        PaginatedResponse, PaginationParams,
        project::{
            AddMemberRequest, CreateProjectRequest, MemberRole, Project, ProjectMember,
            UpdateProjectRequest,
        },
    },
    repositories::{project_repository, user_repository},
};

/// `GET /api/projects`
///
/// Returns all projects the authenticated user can see:
/// public projects plus projects they own.
#[utoipa::path(
    get,
    path = "/api/projects",
    tag = "Projects",
    summary = "List projects",
    description = "Returns all projects visible to the authenticated user (public + owned/member).",
    params(PaginationParams),
    responses(
        (status = 200, description = "Paginated list of projects", body = crate::models::paginated_schemas::PaginatedProjectResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
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
#[utoipa::path(
    post,
    path = "/api/projects",
    tag = "Projects",
    summary = "Create project",
    description = "Creates a new project. The authenticated user becomes the owner.",
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created", body = Project),
        (status = 400, description = "Validation error", body = crate::errors::ErrorResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 409, description = "Identifier already taken", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn create_project(
    State(state): State<AppState>,
    auth: Auth,
    Json(input): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    input.validate()?;

    if project_repository::exists_by_identifier(&state.db, &input.identifier).await? {
        return Err(AppError::Conflict(
            "A project with that identifier already exists.".to_string(),
        ));
    }

    let project = project_repository::create(&state.db, &input, auth.user_id).await?;
    project_repository::add_member(&state.db, project.id, auth.user_id, MemberRole::Admin).await?;
    Ok((StatusCode::CREATED, Json(project)))
}

/// `GET /api/projects/:project_id`
#[utoipa::path(
    get,
    path = "/api/projects/{project_id}",
    tag = "Projects",
    summary = "Get project",
    description = "Returns a single project by ID. Requires visibility access.",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    responses(
        (status = 200, description = "Project details", body = Project),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::errors::ErrorResponse),
        (status = 404, description = "Project not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
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
#[utoipa::path(
    patch,
    path = "/api/projects/{project_id}",
    tag = "Projects",
    summary = "Update project",
    description = "Updates a project's name, description, or visibility. Requires owner or admin role.",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "Project updated", body = Project),
        (status = 400, description = "Validation error", body = crate::errors::ErrorResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::errors::ErrorResponse),
        (status = 404, description = "Project not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn update_project(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Json(input): Json<UpdateProjectRequest>,
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

    let updated = project_repository::update(&state.db, project_id, &input).await?;
    Ok(Json(updated))
}

/// `DELETE /api/projects/:project_id`
///
/// Only the project owner can delete a project.
#[utoipa::path(
    delete,
    path = "/api/projects/{project_id}",
    tag = "Projects",
    summary = "Delete project",
    description = "Deletes a project. Only the project owner can perform this action.",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    responses(
        (status = 204, description = "Project deleted"),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied (not owner)", body = crate::errors::ErrorResponse),
        (status = 404, description = "Project not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
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
#[utoipa::path(
    get,
    path = "/api/projects/{project_id}/members",
    tag = "Projects",
    summary = "List project members",
    description = "Returns the members of a project. Requires visibility access to the project.",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        PaginationParams,
    ),
    responses(
        (status = 200, description = "Paginated list of members", body = crate::models::paginated_schemas::PaginatedProjectMemberResponse),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied", body = crate::errors::ErrorResponse),
        (status = 404, description = "Project not found", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
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
#[utoipa::path(
    post,
    path = "/api/projects/{project_id}/members",
    tag = "Projects",
    summary = "Add project member",
    description = "Adds a user to a project with the specified role. Requires owner or admin role.",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    request_body = AddMemberRequest,
    responses(
        (status = 204, description = "Member added"),
        (status = 401, description = "Not authenticated", body = crate::errors::ErrorResponse),
        (status = 403, description = "Access denied (not admin)", body = crate::errors::ErrorResponse),
        (status = 404, description = "Project or user not found", body = crate::errors::ErrorResponse),
        (status = 409, description = "User is already a member", body = crate::errors::ErrorResponse),
    ),
    security(("bearer" = [])),
)]
pub async fn add_member(
    State(state): State<AppState>,
    auth: Auth,
    Path(project_id): Path<Uuid>,
    Json(input): Json<AddMemberRequest>,
) -> Result<StatusCode, AppError> {
    let project = project_repository::find_by_id(&state.db, project_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Project not found.".to_string()))?;

    let member_role =
        project_repository::find_member_role(&state.db, project.id, auth.user_id).await?;

    if !project.can_admin(auth.user_id, member_role) {
        return Err(AppError::Forbidden);
    }

    if user_repository::find_by_id(&state.db, input.user_id)
        .await?
        .is_none()
    {
        return Err(AppError::NotFound("User not found.".to_string()));
    }

    if project_repository::exists_member(&state.db, project_id, input.user_id).await? {
        return Err(AppError::Conflict(
            "User is already a member of this project.".to_string(),
        ));
    }

    project_repository::add_member(&state.db, project_id, input.user_id, input.role).await?;
    Ok(StatusCode::NO_CONTENT)
}
