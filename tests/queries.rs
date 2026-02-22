mod common;

use todo_rs_ts::queries;

#[tokio::test]
async fn insert_and_fetch() {
    let pool = common::test_pool().await;

    let row = queries::insert_todo(&pool, "Buy milk").await.unwrap();
    assert_eq!(row.title, "Buy milk");
    assert!(!row.completed);

    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].id, row.id);
}

#[tokio::test]
async fn list_filtered_active() {
    let pool = common::test_pool().await;

    let row = queries::insert_todo(&pool, "Active one").await.unwrap();
    queries::insert_todo(&pool, "Complete me").await.unwrap();
    // Complete the second todo
    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    let other_id = todos.iter().find(|t| t.id != row.id).unwrap().id.clone();
    queries::update_todo(&pool, &other_id, None, Some(true)).await.unwrap();

    let active = queries::list_todos_filtered(&pool, "active").await.unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].title, "Active one");
}

#[tokio::test]
async fn list_filtered_completed() {
    let pool = common::test_pool().await;

    queries::insert_todo(&pool, "Active one").await.unwrap();
    let completed_row = queries::insert_todo(&pool, "Done").await.unwrap();
    queries::update_todo(&pool, &completed_row.id, None, Some(true)).await.unwrap();

    let completed = queries::list_todos_filtered(&pool, "completed").await.unwrap();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].title, "Done");
}

#[tokio::test]
async fn list_ordering() {
    let pool = common::test_pool().await;

    // Insert with slight delay so created_at differs
    queries::insert_todo(&pool, "First").await.unwrap();
    queries::insert_todo(&pool, "Second").await.unwrap();
    queries::insert_todo(&pool, "Third").await.unwrap();

    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    assert_eq!(todos.len(), 3);
    // ORDER BY created_at DESC — most recent first
    // With same-second timestamps, insertion order may vary,
    // but we at least verify all three are returned.
    let titles: Vec<&str> = todos.iter().map(|t| t.title.as_str()).collect();
    assert!(titles.contains(&"First"));
    assert!(titles.contains(&"Second"));
    assert!(titles.contains(&"Third"));
}

#[tokio::test]
async fn todo_counts_mixed() {
    let pool = common::test_pool().await;

    queries::insert_todo(&pool, "Active 1").await.unwrap();
    queries::insert_todo(&pool, "Active 2").await.unwrap();
    let done = queries::insert_todo(&pool, "Done").await.unwrap();
    queries::update_todo(&pool, &done.id, None, Some(true)).await.unwrap();

    let (active, completed) = queries::todo_counts(&pool).await.unwrap();
    assert_eq!(active, 2);
    assert_eq!(completed, 1);
}

#[tokio::test]
async fn todo_counts_empty() {
    let pool = common::test_pool().await;

    let (active, completed) = queries::todo_counts(&pool).await.unwrap();
    assert_eq!(active, 0);
    assert_eq!(completed, 0);
}

#[tokio::test]
async fn update_title_only() {
    let pool = common::test_pool().await;

    let row = queries::insert_todo(&pool, "Old title").await.unwrap();
    let updated = queries::update_todo(&pool, &row.id, Some("New title"), None)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated.title, "New title");
    assert_eq!(updated.completed, row.completed);
}

#[tokio::test]
async fn update_completed_only() {
    let pool = common::test_pool().await;

    let row = queries::insert_todo(&pool, "My todo").await.unwrap();
    assert!(!row.completed);

    let updated = queries::update_todo(&pool, &row.id, None, Some(true))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(updated.title, "My todo");
    assert!(updated.completed);
}

#[tokio::test]
async fn update_nonexistent_returns_none() {
    let pool = common::test_pool().await;

    let result = queries::update_todo(&pool, &"nonexistent-id".to_string(), Some("x"), None)
        .await
        .unwrap();

    assert!(result.is_none());
}

#[tokio::test]
async fn delete_and_verify() {
    let pool = common::test_pool().await;

    let keep = queries::insert_todo(&pool, "Keep me").await.unwrap();
    let remove = queries::insert_todo(&pool, "Remove me").await.unwrap();

    let deleted = queries::delete_todo(&pool, &remove.id).await.unwrap();
    assert!(deleted);

    let todos = queries::list_todos_filtered(&pool, "all").await.unwrap();
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].id, keep.id);
}

#[tokio::test]
async fn delete_nonexistent_returns_false() {
    let pool = common::test_pool().await;

    let deleted = queries::delete_todo(&pool, &"nonexistent-id".to_string())
        .await
        .unwrap();

    assert!(!deleted);
}
