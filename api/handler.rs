use axum::routing::{delete, get, patch, post};
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceBuilder;
use vercel_runtime::axum::VercelLayer;
use vercel_runtime::Error;

use todo_rs_ts::handlers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .min_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect(":memory:")
        .await
        .expect("Failed to create SQLite pool");

    // Run migration on every cold start
    sqlx::migrate!("./migrations/sqlite")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let router = Router::new()
        .route("/api/todos", get(handlers::list_todos).post(handlers::create_todo))
        .route(
            "/api/todos/{id}",
            patch(handlers::update_todo).delete(handlers::delete_todo),
        )
        .route("/api/todos/toggle-all", post(handlers::toggle_all))
        .route("/api/todos/completed", delete(handlers::clear_completed))
        .with_state(pool);

    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .service(router);
    vercel_runtime::run(app).await
}
