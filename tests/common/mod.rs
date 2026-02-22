use sqlx::sqlite::SqlitePoolOptions;
use todo_rs_ts::models::DbPool;

pub async fn test_pool() -> DbPool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::migrate!("./migrations/sqlite").run(&pool).await.unwrap();
    pool
}
