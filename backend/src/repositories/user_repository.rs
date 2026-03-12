use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user::User;

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, display_name, created_at, updated_at
         FROM users
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, display_name, created_at, updated_at
         FROM users
         WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

/// Insert a new user row and return the created record.
///
/// The caller is responsible for hashing the password before passing it here.
/// Duplicate email will surface as `AppError::Conflict` via the `From<sqlx::Error>`
/// implementation (PostgreSQL error code 23505).
pub async fn create(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    display_name: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash, display_name)
         VALUES ($1, $2, $3, $4)
         RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(password_hash)
    .bind(display_name)
    .fetch_one(pool)
    .await
}

/// Update mutable user fields. Pass `None` to leave a field unchanged.
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    display_name: Option<&str>,
    password_hash: Option<&str>,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "UPDATE users
         SET display_name  = COALESCE($2, display_name),
             password_hash = COALESCE($3, password_hash),
             updated_at    = now()
         WHERE id = $1
         RETURNING id, email, password_hash, display_name, created_at, updated_at",
    )
    .bind(id)
    .bind(display_name)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}
