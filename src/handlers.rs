use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::*;

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub filter: Option<String>,
}

pub async fn list_todos(
    State(pool): State<DbPool>,
    Query(params): Query<FilterParams>,
) -> Result<Json<TodoListResponse>, AppError> {
    let filter = params.filter.as_deref().unwrap_or("all");

    #[cfg(feature = "postgres")]
    let rows: Vec<TodoRow> = match filter {
        "active" => {
            sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = false ORDER BY created_at DESC")
                .fetch_all(&pool)
                .await?
        }
        "completed" => {
            sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = true ORDER BY created_at DESC")
                .fetch_all(&pool)
                .await?
        }
        _ => {
            sqlx::query_as!(TodoRow, "SELECT * FROM todos ORDER BY created_at DESC")
                .fetch_all(&pool)
                .await?
        }
    };

    #[cfg(feature = "sqlite")]
    let rows: Vec<TodoRow> = match filter {
        "active" => {
            sqlx::query_as!(
                TodoRow,
                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
                   created_at as "created_at!", updated_at as "updated_at!"
                   FROM todos WHERE completed = 0 ORDER BY created_at DESC"#
            )
            .fetch_all(&pool)
            .await?
        }
        "completed" => {
            sqlx::query_as!(
                TodoRow,
                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
                   created_at as "created_at!", updated_at as "updated_at!"
                   FROM todos WHERE completed = 1 ORDER BY created_at DESC"#
            )
            .fetch_all(&pool)
            .await?
        }
        _ => {
            sqlx::query_as!(
                TodoRow,
                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
                   created_at as "created_at!", updated_at as "updated_at!"
                   FROM todos ORDER BY created_at DESC"#
            )
            .fetch_all(&pool)
            .await?
        }
    };

    #[cfg(feature = "postgres")]
    let counts = sqlx::query!(
        r#"SELECT
            COUNT(*) FILTER (WHERE NOT completed) as "active_count!",
            COUNT(*) FILTER (WHERE completed) as "completed_count!"
         FROM todos"#
    )
    .fetch_one(&pool)
    .await?;

    #[cfg(feature = "sqlite")]
    let counts = sqlx::query!(
        r#"SELECT
            COALESCE(SUM(CASE WHEN NOT completed THEN 1 ELSE 0 END), 0) as "active_count!: i64",
            COALESCE(SUM(CASE WHEN completed THEN 1 ELSE 0 END), 0) as "completed_count!: i64"
         FROM todos"#
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(TodoListResponse {
        todos: rows.into_iter().map(Todo::from).collect(),
        active_count: counts.active_count,
        completed_count: counts.completed_count,
    }))
}

pub async fn create_todo(
    State(pool): State<DbPool>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, AppError> {
    let title = req.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::BadRequest("title cannot be empty".into()));
    }
    if title.len() > 500 {
        return Err(AppError::BadRequest("title cannot exceed 500 characters".into()));
    }

    #[cfg(feature = "postgres")]
    let row: TodoRow = sqlx::query_as!(
        TodoRow,
        "INSERT INTO todos (title) VALUES ($1) RETURNING *",
        title
    )
    .fetch_one(&pool)
    .await?;

    #[cfg(feature = "sqlite")]
    let row: TodoRow = {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as!(
            TodoRow,
            r#"INSERT INTO todos (id, title) VALUES (?1, ?2)
               RETURNING id as "id!", title as "title!", completed as "completed!: bool",
               created_at as "created_at!", updated_at as "updated_at!""#,
            id,
            title
        )
        .fetch_one(&pool)
        .await?
    };

    Ok(Json(Todo::from(row)))
}

#[cfg(feature = "postgres")]
pub async fn update_todo(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, AppError> {
    if let Some(ref title) = req.title {
        if title.len() > 500 {
            return Err(AppError::BadRequest("title cannot exceed 500 characters".into()));
        }
    }

    let row: Option<TodoRow> = sqlx::query_as!(
        TodoRow,
        "UPDATE todos SET \
            title = COALESCE($1, title), \
            completed = COALESCE($2, completed), \
            updated_at = now() \
         WHERE id = $3 \
         RETURNING *",
        req.title,
        req.completed,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match row {
        Some(row) => Ok(Json(Todo::from(row))),
        None => Err(AppError::NotFound),
    }
}

#[cfg(feature = "sqlite")]
pub async fn update_todo(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, AppError> {
    if let Some(ref title) = req.title {
        if title.len() > 500 {
            return Err(AppError::BadRequest("title cannot exceed 500 characters".into()));
        }
    }

    let row: Option<TodoRow> = sqlx::query_as!(
        TodoRow,
        r#"UPDATE todos SET
            title = COALESCE(?1, title),
            completed = COALESCE(?2, completed),
            updated_at = datetime('now')
         WHERE id = ?3
         RETURNING id as "id!", title as "title!", completed as "completed!: bool",
         created_at as "created_at!", updated_at as "updated_at!""#,
        req.title,
        req.completed,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match row {
        Some(row) => Ok(Json(Todo::from(row))),
        None => Err(AppError::NotFound),
    }
}

#[cfg(feature = "postgres")]
pub async fn delete_todo(
    State(pool): State<DbPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(()))
}

#[cfg(feature = "sqlite")]
pub async fn delete_todo(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?1", id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(()))
}

pub async fn toggle_all(
    State(pool): State<DbPool>,
) -> Result<Json<()>, AppError> {
    // If all are completed, set all to active; otherwise set all to completed
    #[cfg(feature = "postgres")]
    let active_count = {
        let rec = sqlx::query!(r#"SELECT COUNT(*) as "cnt!" FROM todos WHERE NOT completed"#)
            .fetch_one(&pool)
            .await?;
        rec.cnt
    };

    #[cfg(feature = "sqlite")]
    let active_count = {
        let rec = sqlx::query!(r#"SELECT COUNT(*) as "cnt!: i64" FROM todos WHERE NOT completed"#)
            .fetch_one(&pool)
            .await?;
        rec.cnt
    };

    let new_completed = active_count > 0;

    #[cfg(feature = "postgres")]
    sqlx::query!("UPDATE todos SET completed = $1, updated_at = now()", new_completed)
        .execute(&pool)
        .await?;

    #[cfg(feature = "sqlite")]
    sqlx::query!("UPDATE todos SET completed = ?1, updated_at = datetime('now')", new_completed)
        .execute(&pool)
        .await?;

    Ok(Json(()))
}

pub async fn clear_completed(
    State(pool): State<DbPool>,
) -> Result<Json<()>, AppError> {
    #[cfg(feature = "postgres")]
    sqlx::query!("DELETE FROM todos WHERE completed = true")
        .execute(&pool)
        .await?;

    #[cfg(feature = "sqlite")]
    sqlx::query!("DELETE FROM todos WHERE completed = 1")
        .execute(&pool)
        .await?;

    Ok(Json(()))
}

#[cfg(feature = "test-helpers")]
pub async fn delete_all(
    State(pool): State<DbPool>,
) -> Result<Json<()>, AppError> {
    // No cfg-gating needed: SQL is identical for both backends (unlike queries
    // that use FILTER/CASE, now()/datetime('now'), or $1/?1 placeholders).
    sqlx::query!("DELETE FROM todos")
        .execute(&pool)
        .await?;

    Ok(Json(()))
}
