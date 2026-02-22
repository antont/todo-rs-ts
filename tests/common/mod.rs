use sqlx::SqlitePool;
use todo_rs_ts::models::DbPool;

pub async fn test_pool() -> DbPool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations/sqlite").run(&pool).await.unwrap();
    pool
}
