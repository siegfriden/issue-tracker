use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("missing required environment variable: DATABASE_URL");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let current_version: Option<i64> =
        sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations WHERE success = true")
            .fetch_one(&pool)
            .await
            .expect("failed to query applied migrations");

    let Some(current) = current_version else {
        eprintln!("no applied migrations to revert");
        return;
    };

    sqlx::migrate!("./migrations")
        .undo(&pool, current - 1)
        .await
        .expect("migration revert failed");

    eprintln!("reverted migration version {current}");
}
