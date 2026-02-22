use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::error::AppError;
use crate::models::*;
use crate::queries;

#[derive(Debug, Deserialize)]
pub struct FilterParams {
    pub filter: Option<String>,
}

pub async fn list_todos(
    State(pool): State<DbPool>,
    Query(params): Query<FilterParams>,
) -> Result<Json<TodoListResponse>, AppError> {
    let filter = params.filter.as_deref().unwrap_or("all");

    let rows = queries::list_todos_filtered(&pool, filter).await?;
    let (active_count, completed_count) = queries::todo_counts(&pool).await?;

    Ok(Json(TodoListResponse {
        todos: rows.into_iter().map(Todo::from).collect(),
        active_count,
        completed_count,
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

    let row = queries::insert_todo(&pool, &title).await?;
    Ok(Json(Todo::from(row)))
}

pub async fn update_todo(
    State(pool): State<DbPool>,
    Path(id): Path<TodoId>,
    Json(req): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, AppError> {
    if let Some(ref title) = req.title {
        if title.len() > 500 {
            return Err(AppError::BadRequest("title cannot exceed 500 characters".into()));
        }
    }

    let row = queries::update_todo(&pool, &id, req.title.as_deref(), req.completed).await?;

    match row {
        Some(row) => Ok(Json(Todo::from(row))),
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_todo(
    State(pool): State<DbPool>,
    Path(id): Path<TodoId>,
) -> Result<Json<()>, AppError> {
    let deleted = queries::delete_todo(&pool, &id).await?;

    if !deleted {
        return Err(AppError::NotFound);
    }

    Ok(Json(()))
}

pub async fn toggle_all(
    State(pool): State<DbPool>,
) -> Result<Json<()>, AppError> {
    let active_count = queries::count_active(&pool).await?;
    let new_completed = active_count > 0;
    queries::set_all_completed(&pool, new_completed).await?;
    Ok(Json(()))
}

pub async fn clear_completed(
    State(pool): State<DbPool>,
) -> Result<Json<()>, AppError> {
    queries::delete_completed(&pool).await?;
    Ok(Json(()))
}

#[cfg(feature = "test-helpers")]
pub async fn delete_all(
    State(pool): State<DbPool>,
) -> Result<Json<()>, AppError> {
    queries::delete_all(&pool).await?;
    Ok(Json(()))
}
