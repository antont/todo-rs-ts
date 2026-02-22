#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("features 'postgres' and 'sqlite' are mutually exclusive");

use crate::models::{DbPool, TodoId, TodoRow};

// ---------------------------------------------------------------------------
// list_todos_filtered
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn list_todos_filtered(pool: &DbPool, filter: &str) -> Result<Vec<TodoRow>, sqlx::Error> {
    match filter {
        "active" => {
            sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = false ORDER BY created_at DESC")
                .fetch_all(pool)
                .await
        }
        "completed" => {
            sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = true ORDER BY created_at DESC")
                .fetch_all(pool)
                .await
        }
        _ => {
            sqlx::query_as!(TodoRow, "SELECT * FROM todos ORDER BY created_at DESC")
                .fetch_all(pool)
                .await
        }
    }
}

#[cfg(feature = "sqlite")]
pub async fn list_todos_filtered(pool: &DbPool, filter: &str) -> Result<Vec<TodoRow>, sqlx::Error> {
    match filter {
        "active" => {
            sqlx::query_as!(
                TodoRow,
                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
                   created_at as "created_at!", updated_at as "updated_at!"
                   FROM todos WHERE completed = 0 ORDER BY created_at DESC"#
            )
            .fetch_all(pool)
            .await
        }
        "completed" => {
            sqlx::query_as!(
                TodoRow,
                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
                   created_at as "created_at!", updated_at as "updated_at!"
                   FROM todos WHERE completed = 1 ORDER BY created_at DESC"#
            )
            .fetch_all(pool)
            .await
        }
        _ => {
            sqlx::query_as!(
                TodoRow,
                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
                   created_at as "created_at!", updated_at as "updated_at!"
                   FROM todos ORDER BY created_at DESC"#
            )
            .fetch_all(pool)
            .await
        }
    }
}

// ---------------------------------------------------------------------------
// todo_counts
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn todo_counts(pool: &DbPool) -> Result<(i64, i64), sqlx::Error> {
    let counts = sqlx::query!(
        r#"SELECT
            COUNT(*) FILTER (WHERE NOT completed) as "active_count!",
            COUNT(*) FILTER (WHERE completed) as "completed_count!"
         FROM todos"#
    )
    .fetch_one(pool)
    .await?;
    Ok((counts.active_count, counts.completed_count))
}

#[cfg(feature = "sqlite")]
pub async fn todo_counts(pool: &DbPool) -> Result<(i64, i64), sqlx::Error> {
    let counts = sqlx::query!(
        r#"SELECT
            COALESCE(SUM(CASE WHEN NOT completed THEN 1 ELSE 0 END), 0) as "active_count!: i64",
            COALESCE(SUM(CASE WHEN completed THEN 1 ELSE 0 END), 0) as "completed_count!: i64"
         FROM todos"#
    )
    .fetch_one(pool)
    .await?;
    Ok((counts.active_count, counts.completed_count))
}

// ---------------------------------------------------------------------------
// insert_todo
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn insert_todo(pool: &DbPool, title: &str) -> Result<TodoRow, sqlx::Error> {
    sqlx::query_as!(
        TodoRow,
        "INSERT INTO todos (title) VALUES ($1) RETURNING *",
        title
    )
    .fetch_one(pool)
    .await
}

#[cfg(feature = "sqlite")]
pub async fn insert_todo(pool: &DbPool, title: &str) -> Result<TodoRow, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query_as!(
        TodoRow,
        r#"INSERT INTO todos (id, title) VALUES (?1, ?2)
           RETURNING id as "id!", title as "title!", completed as "completed!: bool",
           created_at as "created_at!", updated_at as "updated_at!""#,
        id,
        title
    )
    .fetch_one(pool)
    .await
}

// ---------------------------------------------------------------------------
// update_todo
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn update_todo(
    pool: &DbPool,
    id: &TodoId,
    title: Option<&str>,
    completed: Option<bool>,
) -> Result<Option<TodoRow>, sqlx::Error> {
    sqlx::query_as!(
        TodoRow,
        "UPDATE todos SET \
            title = COALESCE($1, title), \
            completed = COALESCE($2, completed), \
            updated_at = now() \
         WHERE id = $3 \
         RETURNING *",
        title,
        completed,
        id
    )
    .fetch_optional(pool)
    .await
}

#[cfg(feature = "sqlite")]
pub async fn update_todo(
    pool: &DbPool,
    id: &TodoId,
    title: Option<&str>,
    completed: Option<bool>,
) -> Result<Option<TodoRow>, sqlx::Error> {
    sqlx::query_as!(
        TodoRow,
        r#"UPDATE todos SET
            title = COALESCE(?1, title),
            completed = COALESCE(?2, completed),
            updated_at = datetime('now')
         WHERE id = ?3
         RETURNING id as "id!", title as "title!", completed as "completed!: bool",
         created_at as "created_at!", updated_at as "updated_at!""#,
        title,
        completed,
        id
    )
    .fetch_optional(pool)
    .await
}

// ---------------------------------------------------------------------------
// delete_todo
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn delete_todo(pool: &DbPool, id: &TodoId) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

#[cfg(feature = "sqlite")]
pub async fn delete_todo(pool: &DbPool, id: &TodoId) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = ?1", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

// ---------------------------------------------------------------------------
// count_active
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn count_active(pool: &DbPool) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(r#"SELECT COUNT(*) as "cnt!" FROM todos WHERE NOT completed"#)
        .fetch_one(pool)
        .await?;
    Ok(rec.cnt)
}

#[cfg(feature = "sqlite")]
pub async fn count_active(pool: &DbPool) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(r#"SELECT COUNT(*) as "cnt!: i64" FROM todos WHERE NOT completed"#)
        .fetch_one(pool)
        .await?;
    Ok(rec.cnt)
}

// ---------------------------------------------------------------------------
// set_all_completed
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn set_all_completed(pool: &DbPool, completed: bool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE todos SET completed = $1, updated_at = now()", completed)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(feature = "sqlite")]
pub async fn set_all_completed(pool: &DbPool, completed: bool) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE todos SET completed = ?1, updated_at = datetime('now')", completed)
        .execute(pool)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// delete_completed
// ---------------------------------------------------------------------------

#[cfg(feature = "postgres")]
pub async fn delete_completed(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM todos WHERE completed = true")
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(feature = "sqlite")]
pub async fn delete_completed(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM todos WHERE completed = 1")
        .execute(pool)
        .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// delete_all (test-helpers only)
// ---------------------------------------------------------------------------

#[cfg(feature = "test-helpers")]
pub async fn delete_all(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM todos")
        .execute(pool)
        .await?;
    Ok(())
}
