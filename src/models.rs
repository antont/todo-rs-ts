#[cfg(feature = "postgres")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
#[cfg(feature = "postgres")]
use uuid::Uuid;

#[cfg(feature = "postgres")]
pub type DbPool = sqlx::PgPool;
#[cfg(feature = "sqlite")]
pub type DbPool = sqlx::SqlitePool;

#[cfg(feature = "postgres")]
pub type DbId = Uuid;
#[cfg(feature = "sqlite")]
pub type DbId = String;

#[cfg(feature = "postgres")]
pub type DbTimestamp = DateTime<Utc>;
#[cfg(feature = "sqlite")]
pub type DbTimestamp = String;

pub type TodoId = DbId;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TodoRow {
    pub id: DbId,
    pub title: String,
    pub completed: bool,
    pub created_at: DbTimestamp,
    pub updated_at: DbTimestamp,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct TodoListResponse {
    pub todos: Vec<Todo>,
    pub active_count: i32,
    pub completed_count: i32,
}

#[derive(Debug, Deserialize)]
#[typeshare]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[cfg(feature = "postgres")]
fn id_to_string(id: DbId) -> String {
    id.to_string()
}
#[cfg(feature = "sqlite")]
fn id_to_string(id: DbId) -> String {
    id
}

#[cfg(feature = "postgres")]
fn timestamp_to_string(ts: DbTimestamp) -> String {
    ts.to_rfc3339()
}
#[cfg(feature = "sqlite")]
fn timestamp_to_string(ts: DbTimestamp) -> String {
    // SQLite datetime('now') returns "YYYY-MM-DD HH:MM:SS" in UTC.
    // Normalize to RFC 3339 to match the postgres output format.
    ts.replacen(' ', "T", 1) + "+00:00"
}

impl From<TodoRow> for Todo {
    fn from(row: TodoRow) -> Self {
        Self {
            id: id_to_string(row.id),
            title: row.title,
            completed: row.completed,
            created_at: timestamp_to_string(row.created_at),
            updated_at: timestamp_to_string(row.updated_at),
        }
    }
}
