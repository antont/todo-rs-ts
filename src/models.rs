#[cfg(feature = "postgres")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[cfg(feature = "postgres")]
use uuid::Uuid;

#[cfg(feature = "postgres")]
pub type DbPool = sqlx::PgPool;
#[cfg(feature = "sqlite")]
pub type DbPool = sqlx::SqlitePool;

#[cfg(feature = "sqlite")]
type DbTimestamp = String;

#[cfg(feature = "postgres")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TodoRow {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "sqlite")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TodoRow {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TodoListResponse {
    pub todos: Vec<Todo>,
    #[ts(type = "number")]
    pub active_count: i64,
    #[ts(type = "number")]
    pub completed_count: i64,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[cfg(feature = "postgres")]
impl From<TodoRow> for Todo {
    fn from(row: TodoRow) -> Self {
        Self {
            id: row.id.to_string(),
            title: row.title,
            completed: row.completed,
            created_at: row.created_at.to_rfc3339(),
            updated_at: row.updated_at.to_rfc3339(),
        }
    }
}

#[cfg(feature = "sqlite")]
fn timestamp_to_string(ts: DbTimestamp) -> String {
    // SQLite datetime('now') returns "YYYY-MM-DD HH:MM:SS" in UTC.
    // Normalize to RFC 3339 to match the postgres output format.
    ts.replacen(' ', "T", 1) + "+00:00"
}

#[cfg(feature = "sqlite")]
impl From<TodoRow> for Todo {
    fn from(row: TodoRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: timestamp_to_string(row.created_at),
            updated_at: timestamp_to_string(row.updated_at),
        }
    }
}
