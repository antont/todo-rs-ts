mod common;

use axum::extract::State;
use axum::Json;
use todo_rs_ts::handlers;
use todo_rs_ts::models::*;
use todo_rs_ts::queries;

async fn create(pool: &DbPool, title: &str) -> Todo {
    handlers::create_todo(
        State(pool.clone()),
        Json(CreateTodoRequest { title: title.into() }),
    )
    .await
    .unwrap()
    .0
}

#[tokio::test]
async fn toggle_all_marks_active_as_completed() {
    let pool = common::test_pool().await;
    create(&pool, "One").await;
    create(&pool, "Two").await;

    let _ = handlers::toggle_all(State(pool.clone())).await.unwrap();

    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    assert!(todos.iter().all(|t| t.completed));
}

#[tokio::test]
async fn toggle_all_marks_completed_as_active() {
    let pool = common::test_pool().await;
    create(&pool, "One").await;
    create(&pool, "Two").await;

    // Complete all
    let _ = handlers::toggle_all(State(pool.clone())).await.unwrap();
    // Toggle back to active
    let _ = handlers::toggle_all(State(pool.clone())).await.unwrap();

    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    assert!(todos.iter().all(|t| !t.completed));
}

#[tokio::test]
async fn toggle_all_mixed_state() {
    let pool = common::test_pool().await;
    let todo1 = create(&pool, "Active").await;
    create(&pool, "Also active").await;

    // Complete only one
    queries::update_todo(&pool, &todo1.id, None, Some(true))
        .await
        .unwrap();

    // toggle_all with mixed state (some active) → all completed
    let _ = handlers::toggle_all(State(pool.clone())).await.unwrap();

    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    assert!(todos.iter().all(|t| t.completed));
}

// NOTE: toggle_all is NOT atomic. It performs count_active() then set_all_completed()
// as two separate queries (see handlers.rs:78-84). Under concurrent Postgres access,
// a new todo inserted between the two queries could receive the wrong `completed` value.
// This is acceptable for a TodoMVC app but would need a transaction in production.
// SQLite serializes all writes so this race cannot be reproduced here.
