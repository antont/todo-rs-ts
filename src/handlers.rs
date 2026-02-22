use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::*;

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub filter: Option<String>,
}

pub async fn list_todos(
    State(pool): State<PgPool>,
    Query(params): Query<FilterParams>,
) -> Result<Json<TodoListResponse>, AppError> {
    let filter = params.filter.as_deref().unwrap_or("all");

    let rows: Vec<TodoRow> = match filter {
        "active" => {
            sqlx::query_as("SELECT * FROM todos WHERE completed = false ORDER BY created_at DESC")
                .fetch_all(&pool)
                .await?
        }
        "completed" => {
            sqlx::query_as(
                "SELECT * FROM todos WHERE completed = true ORDER BY created_at DESC",
            )
            .fetch_all(&pool)
            .await?
        }
        _ => {
            sqlx::query_as("SELECT * FROM todos ORDER BY created_at DESC")
                .fetch_all(&pool)
                .await?
        }
    };

    let counts: (i64, i64) = sqlx::query_as(
        "SELECT \
            COUNT(*) FILTER (WHERE NOT completed), \
            COUNT(*) FILTER (WHERE completed) \
         FROM todos",
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(TodoListResponse {
        todos: rows.into_iter().map(Todo::from).collect(),
        active_count: counts.0,
        completed_count: counts.1,
    }))
}

pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, AppError> {
    let title = req.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::BadRequest("title cannot be empty".into()));
    }

    let row: TodoRow = sqlx::query_as(
        "INSERT INTO todos (title) VALUES ($1) RETURNING *",
    )
    .bind(&title)
    .fetch_one(&pool)
    .await?;

    Ok(Json(Todo::from(row)))
}

pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, AppError> {
    let row: TodoRow = sqlx::query_as(
        "UPDATE todos SET \
            title = COALESCE($1, title), \
            completed = COALESCE($2, completed), \
            updated_at = now() \
         WHERE id = $3 \
         RETURNING *",
    )
    .bind(&req.title)
    .bind(req.completed)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(Todo::from(row)))
}

pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(()))
}

pub async fn toggle_all(
    State(pool): State<PgPool>,
) -> Result<Json<()>, AppError> {
    // If all are completed, set all to active; otherwise set all to completed
    let (active_count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM todos WHERE NOT completed")
            .fetch_one(&pool)
            .await?;

    let new_completed = active_count > 0;

    sqlx::query("UPDATE todos SET completed = $1, updated_at = now()")
        .bind(new_completed)
        .execute(&pool)
        .await?;

    Ok(Json(()))
}

pub async fn clear_completed(
    State(pool): State<PgPool>,
) -> Result<Json<()>, AppError> {
    sqlx::query("DELETE FROM todos WHERE completed = true")
        .execute(&pool)
        .await?;

    Ok(Json(()))
}

#[cfg(feature = "test-helpers")]
pub async fn delete_all(
    State(pool): State<PgPool>,
) -> Result<Json<()>, AppError> {
    sqlx::query("DELETE FROM todos")
        .execute(&pool)
        .await?;

    Ok(Json(()))
}
