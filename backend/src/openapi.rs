use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Issue Tracker API",
        version = "0.1.0",
        description = "REST API for a Redmine-like issue tracker. Supports projects, issues, comments, and role-based access control.",
    ),
    paths(
        // Health
        crate::handlers::health_handler::get_liveness,
        crate::handlers::health_handler::get_readiness,
        // Auth
        crate::handlers::auth_handler::register,
        crate::handlers::auth_handler::login,
        crate::handlers::auth_handler::refresh,
        // Users
        crate::handlers::user_handler::get_me,
        crate::handlers::user_handler::update_me,
        // Projects
        crate::handlers::project_handler::list_projects,
        crate::handlers::project_handler::create_project,
        crate::handlers::project_handler::get_project,
        crate::handlers::project_handler::update_project,
        crate::handlers::project_handler::delete_project,
        crate::handlers::project_handler::list_members,
        crate::handlers::project_handler::add_member,
        // Issues
        crate::handlers::issue_handler::list_issues,
        crate::handlers::issue_handler::create_issue,
        crate::handlers::issue_handler::get_issue,
        crate::handlers::issue_handler::update_issue,
        crate::handlers::issue_handler::delete_issue,
        // Comments
        crate::handlers::comment_handler::list_comments,
        crate::handlers::comment_handler::create_comment,
        crate::handlers::comment_handler::delete_comment,
    ),
    components(schemas(
        // Error
        crate::errors::ErrorResponse,
        // Health
        crate::handlers::health_handler::ReadinessResponse,
        // Users
        crate::models::user::UserResponse,
        crate::models::user::RegisterRequest,
        crate::models::user::LoginRequest,
        crate::models::user::UpdateUserRequest,
        // Projects
        crate::models::project::Project,
        crate::models::project::CreateProjectRequest,
        crate::models::project::UpdateProjectRequest,
        crate::models::project::MemberRole,
        crate::models::project::ProjectMember,
        crate::models::project::AddMemberRequest,
        // Issues
        crate::models::issue::Issue,
        crate::models::issue::CreateIssueRequest,
        crate::models::issue::UpdateIssueRequest,
        // Comments
        crate::models::comment::Comment,
        crate::models::comment::CreateCommentRequest,
        // Paginated responses
        crate::models::paginated_schemas::PaginatedProjectResponse,
        crate::models::paginated_schemas::PaginatedProjectMemberResponse,
        crate::models::paginated_schemas::PaginatedIssueResponse,
        crate::models::paginated_schemas::PaginatedCommentResponse,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Auth", description = "Authentication (register, login, token refresh)"),
        (name = "Users", description = "User profile management"),
        (name = "Projects", description = "Project and membership management"),
        (name = "Issues", description = "Issue tracking"),
        (name = "Comments", description = "Issue comments"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            let mut http = Http::new(HttpAuthScheme::Bearer);
            http.bearer_format = Some("JWT".to_string());
            components.add_security_scheme("bearer", SecurityScheme::Http(http));
        }
    }
}
