use sqlx::PgPool;
use uuid::Uuid;

use crate::models::PaginationParams;
use crate::models::comment::Comment;

pub async fn find_by_issue(
    pool: &PgPool,
    issue_id: Uuid,
    pagination: &PaginationParams,
) -> Result<(Vec<Comment>, i64), sqlx::Error> {
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT id, issue_id, author_id, body, created_at, updated_at \
         FROM comments WHERE issue_id = $1 \
         ORDER BY created_at ASC LIMIT $2 OFFSET $3",
    )
    .bind(issue_id)
    .bind(pagination.limit())
    .bind(pagination.offset())
    .fetch_all(pool)
    .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM comments WHERE issue_id = $1")
        .bind(issue_id)
        .fetch_one(pool)
        .await?;

    Ok((comments, total))
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Comment>, sqlx::Error> {
    sqlx::query_as::<_, Comment>(
        "SELECT id, issue_id, author_id, body, created_at, updated_at \
         FROM comments WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    issue_id: Uuid,
    author_id: Uuid,
    body: &str,
) -> Result<Comment, sqlx::Error> {
    sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (issue_id, author_id, body) \
         VALUES ($1, $2, $3) \
         RETURNING id, issue_id, author_id, body, created_at, updated_at",
    )
    .bind(issue_id)
    .bind(author_id)
    .bind(body)
    .fetch_one(pool)
    .await
}

/// Returns `true` if a row was deleted.
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
}
