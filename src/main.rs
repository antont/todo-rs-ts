use axum::routing::{delete, get, patch, post};
use axum::Router;
use todo_rs_ts::handlers;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);

    #[cfg(feature = "postgres")]
    let pool = {
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database")
    };

    #[cfg(feature = "sqlite")]
    let pool = {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| ":memory:".to_string());
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");
        sqlx::migrate!("./migrations/sqlite")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        pool
    };

    let cors_origin = std::env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    let cors = CorsLayer::new()
        .allow_origin([
            cors_origin.parse().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/todos", get(handlers::list_todos).post(handlers::create_todo))
        .route("/api/todos/{id}", patch(handlers::update_todo))
        .route("/api/todos/{id}", delete(handlers::delete_todo))
        .route("/api/todos/toggle-all", post(handlers::toggle_all))
        .route("/api/todos/completed", delete(handlers::clear_completed));

    #[cfg(feature = "test-helpers")]
    let app = {
        tracing::info!("test-helpers feature enabled: DELETE /api/test/cleanup is available");
        app.route("/api/test/cleanup", delete(handlers::delete_all))
    };

    let app = app
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind");

    tracing::info!("Listening on http://0.0.0.0:{port}");
    axum::serve(listener, app).await.unwrap();
}
