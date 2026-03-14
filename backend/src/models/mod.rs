pub mod comment;
pub mod issue;
pub mod project;
pub mod user;

use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

/// Serde deserializer for `Option<Option<T>>` fields (nullable PATCH columns).
///
/// - Field absent → outer `None` (via `#[serde(default)]`)
/// - Field is `null` → `Some(None)` (explicitly clear)
/// - Field has value → `Some(Some(v))` (set to value)
pub(super) mod nullable {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Some)
    }
}

/// Query parameters for paginated list endpoints.
///
/// Both fields are optional; defaults are applied when absent.
/// `per_page` is capped at 100 to prevent runaway queries.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(25).min(100)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.limit()
    }
}

/// Standard envelope for paginated list responses.
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, params: &PaginationParams) -> Self {
        Self {
            data,
            total,
            page: params.page(),
            limit: params.limit(),
        }
    }
}

/// Concrete paginated response schemas for OpenAPI documentation.
/// These mirror `PaginatedResponse<T>` for specific entity types.
pub mod paginated_schemas {
    use serde::Serialize;
    use utoipa::ToSchema;

    #[derive(Serialize, ToSchema)]
    pub struct PaginatedProjectResponse {
        pub data: Vec<super::project::Project>,
        pub total: i64,
        pub page: i64,
        pub limit: i64,
    }

    #[derive(Serialize, ToSchema)]
    pub struct PaginatedProjectMemberResponse {
        pub data: Vec<super::project::ProjectMember>,
        pub total: i64,
        pub page: i64,
        pub limit: i64,
    }

    #[derive(Serialize, ToSchema)]
    pub struct PaginatedIssueResponse {
        pub data: Vec<super::issue::Issue>,
        pub total: i64,
        pub page: i64,
        pub limit: i64,
    }

    #[derive(Serialize, ToSchema)]
    pub struct PaginatedCommentResponse {
        pub data: Vec<super::comment::Comment>,
        pub total: i64,
        pub page: i64,
        pub limit: i64,
    }
}
