use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::models::{
    PaginationParams,
    project::{CreateProjectRequest, MemberRole, Project, ProjectMember, UpdateProjectRequest},
};

/// Return all projects visible to `user_id`:
///   - all public projects, OR
///   - private projects owned by this user, OR
///   - projects where the user is a member.
pub async fn find_all_accessible(
    pool: &PgPool,
    user_id: Uuid,
    pagination: &PaginationParams,
) -> Result<(Vec<Project>, i64), sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
        "SELECT id, name, identifier, description, is_public, owner_id, created_at, updated_at \
         FROM projects WHERE is_public = true OR owner_id = ",
    );
    qb.push_bind(user_id);
    qb.push(" OR id IN (SELECT project_id FROM project_members WHERE user_id = ");
    qb.push_bind(user_id);
    qb.push(")");
    qb.push(" ORDER BY created_at DESC LIMIT ");
    qb.push_bind(pagination.limit());
    qb.push(" OFFSET ");
    qb.push_bind(pagination.offset());

    let projects = qb.build_query_as::<Project>().fetch_all(pool).await?;

    let mut count_qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("SELECT COUNT(*) FROM projects WHERE is_public = true OR owner_id = ");
    count_qb.push_bind(user_id);
    count_qb.push(" OR id IN (SELECT project_id FROM project_members WHERE user_id = ");
    count_qb.push_bind(user_id);
    count_qb.push(")");
    let total: i64 = count_qb.build_query_scalar().fetch_one(pool).await?;

    Ok((projects, total))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "SELECT id, name, identifier, description, is_public, owner_id, created_at, updated_at
         FROM projects
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn exists_by_identifier(pool: &PgPool, identifier: &str) -> Result<bool, sqlx::Error> {
    let exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM projects WHERE identifier = $1)")
        .bind(identifier)
        .fetch_one(pool)
        .await?;
    Ok(exists.unwrap_or(false))
}

pub async fn exists_member(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let exists: Option<bool> = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM project_members WHERE project_id = $1 AND user_id = $2)")
        .bind(project_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(exists.unwrap_or(false))
}

pub async fn create(
    pool: &PgPool,
    input: &CreateProjectRequest,
    owner_id: Uuid,
) -> Result<Project, sqlx::Error> {
    sqlx::query_as::<_, Project>(
        "INSERT INTO projects (name, identifier, description, is_public, owner_id)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, name, identifier, description, is_public, owner_id, created_at, updated_at",
    )
    .bind(&input.name)
    .bind(&input.identifier)
    .bind(&input.description)
    .bind(input.is_public)
    .bind(owner_id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateProjectRequest,
) -> Result<Project, sqlx::Error> {
    let mut builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE projects SET updated_at = now()");

    if let Some(v) = input.name.as_deref() {
        builder.push(", name = ").push_bind(v);
    }
    if let Some(v) = input.description.as_deref() {
        builder.push(", description = ").push_bind(v);
    }
    if let Some(v) = input.is_public {
        builder.push(", is_public = ").push_bind(v);
    }

    builder.push(" WHERE id = ").push_bind(id);
    builder.push(
        " RETURNING id, name, identifier, description, is_public, owner_id, created_at, updated_at",
    );

    builder.build_query_as::<Project>().fetch_one(pool).await
}

/// Returns `true` if a row was deleted, `false` if the project was not found.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query("DELETE FROM projects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
}

/// Returns a paginated list of members for a project, ordered by join date.
pub async fn find_members(
    pool: &PgPool,
    project_id: Uuid,
    pagination: &PaginationParams,
) -> Result<(Vec<ProjectMember>, i64), sqlx::Error> {
    let members = sqlx::query_as::<_, ProjectMember>(
        "SELECT project_id, user_id, role, joined_at \
         FROM project_members WHERE project_id = $1 \
         ORDER BY joined_at LIMIT $2 OFFSET $3",
    )
    .bind(project_id)
    .bind(pagination.limit())
    .bind(pagination.offset())
    .fetch_all(pool)
    .await?;

    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM project_members WHERE project_id = $1")
            .bind(project_id)
            .fetch_one(pool)
            .await?;

    Ok((members, total))
}

/// Returns the role of a user in a project, or `None` if they are not a member.
pub async fn find_member_role(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
) -> Result<Option<MemberRole>, sqlx::Error> {
    sqlx::query_scalar("SELECT role FROM project_members WHERE project_id = $1 AND user_id = $2")
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
}

/// Adds a user as a member of a project with the given role.
pub async fn add_member(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
    role: MemberRole,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO project_members (project_id, user_id, role) VALUES ($1, $2, $3)")
        .bind(project_id)
        .bind(user_id)
        .bind(role)
        .execute(pool)
        .await?;
    Ok(())
}
