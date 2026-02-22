mod common;

use todo_rs_ts::models::{Todo, TodoRow};

#[test]
fn todorow_to_todo_maps_fields() {
    let row = TodoRow {
        id: "abc-123".to_string(),
        title: "Buy milk".to_string(),
        completed: false,
        created_at: "2024-01-15 10:30:00".to_string(),
        updated_at: "2024-01-15 11:00:00".to_string(),
    };

    let todo = Todo::from(row);

    assert_eq!(todo.id, "abc-123");
    assert_eq!(todo.title, "Buy milk");
    assert!(!todo.completed);
    assert!(todo.created_at.contains("2024-01-15"));
    assert!(todo.updated_at.contains("2024-01-15"));
}

#[test]
fn sqlite_timestamp_normalized_to_rfc3339() {
    let row = TodoRow {
        id: "id-1".to_string(),
        title: "t".to_string(),
        completed: false,
        created_at: "2024-01-15 10:30:00".to_string(),
        updated_at: "2024-01-15 10:30:00".to_string(),
    };

    let todo = Todo::from(row);

    // SQLite "YYYY-MM-DD HH:MM:SS" → "YYYY-MM-DDTHH:MM:SS+00:00"
    assert_eq!(todo.created_at, "2024-01-15T10:30:00+00:00");
}

#[test]
fn sqlite_timestamp_with_fractional_seconds() {
    let row = TodoRow {
        id: "id-2".to_string(),
        title: "t".to_string(),
        completed: false,
        created_at: "2024-01-15 10:30:00.123".to_string(),
        updated_at: "2024-01-15 10:30:00.123".to_string(),
    };

    let todo = Todo::from(row);

    // Subsecond precision should be preserved through the conversion
    assert_eq!(todo.created_at, "2024-01-15T10:30:00.123+00:00");
}

#[test]
fn id_passthrough() {
    let original_id = "550e8400-e29b-41d4-a716-446655440000".to_string();
    let row = TodoRow {
        id: original_id.clone(),
        title: "t".to_string(),
        completed: false,
        created_at: "2024-01-01 00:00:00".to_string(),
        updated_at: "2024-01-01 00:00:00".to_string(),
    };

    let todo = Todo::from(row);

    assert_eq!(todo.id, original_id);
}

#[test]
fn completed_bool_preserved() {
    let make_row = |completed: bool| TodoRow {
        id: "id".to_string(),
        title: "t".to_string(),
        completed,
        created_at: "2024-01-01 00:00:00".to_string(),
        updated_at: "2024-01-01 00:00:00".to_string(),
    };

    assert!(Todo::from(make_row(true)).completed);
    assert!(!Todo::from(make_row(false)).completed);
}
