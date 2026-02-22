mod common;

use axum::extract::{Path, State};
use axum::Json;
use todo_rs_ts::handlers;
use todo_rs_ts::models::*;

#[tokio::test]
async fn create_trims_whitespace() {
    let pool = common::test_pool().await;

    let result = handlers::create_todo(
        State(pool),
        Json(CreateTodoRequest { title: "  hello  ".into() }),
    )
    .await
    .unwrap();

    assert_eq!(result.0.title, "hello");
}

#[tokio::test]
async fn create_rejects_empty() {
    let pool = common::test_pool().await;

    let result = handlers::create_todo(
        State(pool),
        Json(CreateTodoRequest { title: "   ".into() }),
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn create_rejects_too_long() {
    let pool = common::test_pool().await;
    let long_title = "x".repeat(501);

    let result = handlers::create_todo(
        State(pool),
        Json(CreateTodoRequest { title: long_title }),
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn update_does_not_trim() {
    let pool = common::test_pool().await;

    // Create a todo first
    let created = handlers::create_todo(
        State(pool.clone()),
        Json(CreateTodoRequest { title: "original".into() }),
    )
    .await
    .unwrap();

    // Update with whitespace-padded title — current behavior: NOT trimmed
    let updated = handlers::update_todo(
        State(pool),
        Path(created.0.id.clone()),
        Json(UpdateTodoRequest {
            title: Some("  padded  ".into()),
            completed: None,
        }),
    )
    .await
    .unwrap();

    assert_eq!(updated.0.title, "  padded  ");
}

#[tokio::test]
async fn update_rejects_too_long() {
    let pool = common::test_pool().await;

    let created = handlers::create_todo(
        State(pool.clone()),
        Json(CreateTodoRequest { title: "original".into() }),
    )
    .await
    .unwrap();

    let result = handlers::update_todo(
        State(pool),
        Path(created.0.id.clone()),
        Json(UpdateTodoRequest {
            title: Some("x".repeat(501)),
            completed: None,
        }),
    )
    .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn delete_nonexistent_returns_error() {
    let pool = common::test_pool().await;

    let result = handlers::delete_todo(
        State(pool),
        Path("nonexistent-id".to_string()),
    )
    .await;

    assert!(result.is_err());
}
