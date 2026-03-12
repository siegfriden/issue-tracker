use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::models::{
    PaginationParams,
    issue::{CreateIssueRequest, Issue, IssueFilters, UpdateIssueRequest},
};

/// List issues for a project with optional filters and pagination.
///
/// Returns `(rows, total_count)` so the handler can build a paginated response.
pub async fn find_by_project(
    pool: &PgPool,
    project_id: Uuid,
    filters: &IssueFilters,
    pagination: &PaginationParams,
) -> Result<(Vec<Issue>, i64), sqlx::Error> {
    // ── Data query ──────────────────────────────────────────────────────────
    let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
        "SELECT id, project_id, author_id, assignee_id, subject, description, \
         issue_type, status, priority, due_date, created_at, updated_at \
         FROM issues WHERE project_id = ",
    );
    qb.push_bind(project_id);
    push_filters(&mut qb, filters);
    qb.push(" ORDER BY created_at DESC LIMIT ");
    qb.push_bind(pagination.limit());
    qb.push(" OFFSET ");
    qb.push_bind(pagination.offset());

    let issues = qb.build_query_as::<Issue>().fetch_all(pool).await?;

    // ── Count query (same WHERE clause, no pagination) ───────────────────────
    let mut count_qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("SELECT COUNT(*) FROM issues WHERE project_id = ");
    count_qb.push_bind(project_id);
    push_filters(&mut count_qb, filters);

    let total: i64 = count_qb.build_query_scalar().fetch_one(pool).await?;

    Ok((issues, total))
}

/// Append optional filter conditions to a query builder.
fn push_filters(qb: &mut QueryBuilder<sqlx::Postgres>, filters: &IssueFilters) {
    if let Some(ref status) = filters.status {
        qb.push(" AND status = ");
        qb.push_bind(status.clone());
    }
    if let Some(ref priority) = filters.priority {
        qb.push(" AND priority = ");
        qb.push_bind(priority.clone());
    }
    if let Some(assignee_id) = filters.assignee_id {
        qb.push(" AND assignee_id = ");
        qb.push_bind(assignee_id);
    }
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Issue>, sqlx::Error> {
    sqlx::query_as::<_, Issue>(
        "SELECT id, project_id, author_id, assignee_id, subject, description, \
         issue_type, status, priority, due_date, created_at, updated_at \
         FROM issues WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    input: &CreateIssueRequest,
    project_id: Uuid,
    author_id: Uuid,
) -> Result<Issue, sqlx::Error> {
    sqlx::query_as::<_, Issue>(
        "INSERT INTO issues \
         (project_id, author_id, assignee_id, subject, description, \
          issue_type, status, priority, due_date) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
         RETURNING id, project_id, author_id, assignee_id, subject, description, \
                   issue_type, status, priority, due_date, created_at, updated_at",
    )
    .bind(project_id)
    .bind(author_id)
    .bind(input.assignee_id)
    .bind(&input.subject)
    .bind(&input.description)
    .bind(&input.issue_type)
    .bind(&input.status)
    .bind(&input.priority)
    .bind(input.due_date)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    input: &UpdateIssueRequest,
) -> Result<Issue, sqlx::Error> {
    let mut builder: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE issues SET updated_at = now()");

    if let Some(v) = input.subject.as_deref() {
        builder.push(", subject = ").push_bind(v);
    }
    if let Some(v) = input.issue_type.as_deref() {
        builder.push(", issue_type = ").push_bind(v);
    }
    if let Some(v) = input.status.as_deref() {
        builder.push(", status = ").push_bind(v);
    }
    if let Some(v) = input.priority.as_deref() {
        builder.push(", priority = ").push_bind(v);
    }
    if let Some(v) = input.description.as_deref() {
        builder.push(", description = ").push_bind(v);
    }
    if let Some(ref v) = input.assignee_id {
        builder.push(", assignee_id = ").push_bind(*v);
    }
    if let Some(ref v) = input.due_date {
        builder.push(", due_date = ").push_bind(*v);
    }

    builder.push(" WHERE id = ").push_bind(id);
    builder.push(
        " RETURNING id, project_id, author_id, assignee_id, subject, description, \
         issue_type, status, priority, due_date, created_at, updated_at",
    );

    builder.build_query_as::<Issue>().fetch_one(pool).await
}

/// Returns `true` if a row was deleted.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query("DELETE FROM issues WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
}
