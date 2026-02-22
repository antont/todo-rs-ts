# Why we don't use a macro to deduplicate queries.rs

Quite a lot. Here's the breakdown by function:

| Function | Differs only in... | Structural duplication |
|---|---|---|
| `list_todos_filtered` | SQL syntax (`SELECT *` vs explicit column casts, `false`/`true` vs `0`/`1`) | Entire match block + 3 arms duplicated |
| `todo_counts` | SQL (`FILTER (WHERE ...)` vs `COALESCE(SUM(CASE ...))`, type cast) | All wrapper code identical |
| `insert_todo` | SQL placeholders, sqlite needs client-side UUID + column casts | Wrapper code identical |
| `update_todo` | SQL (`now()` vs `datetime('now')`, `$N` vs `?N`, `RETURNING *` vs column casts) | Signature + `.fetch_optional` + `.await` identical |
| `delete_todo` | `$1` vs `?1` | Everything else is byte-for-byte identical |
| `count_active` | `"cnt!"` vs `"cnt!: i64"` | Everything else identical |
| `set_all_completed` | `$1` vs `?1`, `now()` vs `datetime('now')` | Everything else identical |
| `delete_completed` | `true` vs `1` | Everything else identical |

**7 of 8 function pairs share identical signatures and wrapper logic** — only the SQL string literal differs. The root cause is that `sqlx::query!` / `query_as!` are macros that need the SQL at compile time, so you can't pass the query string as a runtime variable or factor it through a helper.

The only ways to reduce this further would be:
1. **Drop compile-time verification** — use `sqlx::query_as_unchecked!` or runtime `sqlx::query_as()` with string arguments, so you could parameterize the SQL. Loses the main benefit of the `query!` macros.
2. **A declarative macro** — e.g. `define_query_fn!` that stamps out the cfg-gated pair from a template. Reduces line count but adds macro complexity and may confuse IDE tooling / sqlx offline prepare.
3. **Accept it** — this is the intentional tradeoff of compile-time verified SQL with two backends. The duplication is confined to one file and each pair is trivially auditable.

## What the macro approach (option 2) would look like

Here's how a `dual_fn!` macro approach would look. The most practical version stamps out the cfg-gated pair from a shared signature, keeping the bodies explicit:

```rust
// The macro — saves repeated signatures, cfg attrs, and prevents them drifting
macro_rules! dual_fn {
    (
        $(#[$attr:meta])*
        pub async fn $name:ident($($sig:tt)*) -> Result<$ret:ty, sqlx::Error>
        [postgres] { $($pg:tt)* }
        [sqlite]   { $($sq:tt)* }
    ) => {
        $(#[$attr])*
        #[cfg(feature = "postgres")]
        pub async fn $name($($sig)*) -> Result<$ret, sqlx::Error> { $($pg)* }

        $(#[$attr])*
        #[cfg(feature = "sqlite")]
        pub async fn $name($($sig)*) -> Result<$ret, sqlx::Error> { $($sq)* }
    };
}
```

Applied to the full file:

```diff
--- a/src/queries.rs
+++ b/src/queries.rs
@@ -1,256 +1,175 @@
 #[cfg(all(feature = "postgres", feature = "sqlite"))]
 compile_error!("features 'postgres' and 'sqlite' are mutually exclusive");

 use crate::models::{DbPool, TodoId, TodoRow};

-// ---------------------------------------------------------------------------
-// list_todos_filtered
-// ---------------------------------------------------------------------------
+macro_rules! dual_fn {
+    (
+        $(#[$attr:meta])*
+        pub async fn $name:ident($($sig:tt)*) -> Result<$ret:ty, sqlx::Error>
+        [postgres] { $($pg:tt)* }
+        [sqlite]   { $($sq:tt)* }
+    ) => {
+        $(#[$attr])*
+        #[cfg(feature = "postgres")]
+        pub async fn $name($($sig)*) -> Result<$ret, sqlx::Error> { $($pg)* }
+        $(#[$attr])*
+        #[cfg(feature = "sqlite")]
+        pub async fn $name($($sig)*) -> Result<$ret, sqlx::Error> { $($sq)* }
+    };
+}

-#[cfg(feature = "postgres")]
-pub async fn list_todos_filtered(pool: &DbPool, filter: &str) -> Result<Vec<TodoRow>, sqlx::Error> {
-    match filter {
-        "active" => {
-            sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = false ORDER BY created_at DESC")
-                .fetch_all(pool)
-                .await
-        }
-        "completed" => {
-            sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = true ORDER BY created_at DESC")
-                .fetch_all(pool)
-                .await
-        }
-        _ => {
-            sqlx::query_as!(TodoRow, "SELECT * FROM todos ORDER BY created_at DESC")
-                .fetch_all(pool)
-                .await
+dual_fn! {
+    pub async fn list_todos_filtered(pool: &DbPool, filter: &str) -> Result<Vec<TodoRow>, sqlx::Error>
+    [postgres] {
+        match filter {
+            "active" => sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = false ORDER BY created_at DESC").fetch_all(pool).await,
+            "completed" => sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE completed = true ORDER BY created_at DESC").fetch_all(pool).await,
+            _ => sqlx::query_as!(TodoRow, "SELECT * FROM todos ORDER BY created_at DESC").fetch_all(pool).await,
         }
     }
-}
-
-#[cfg(feature = "sqlite")]
-pub async fn list_todos_filtered(pool: &DbPool, filter: &str) -> Result<Vec<TodoRow>, sqlx::Error> {
-    match filter {
-        "active" => {
-            sqlx::query_as!(
-                TodoRow,
-                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
-                   created_at as "created_at!", updated_at as "updated_at!"
-                   FROM todos WHERE completed = 0 ORDER BY created_at DESC"#
-            )
-            .fetch_all(pool)
-            .await
-        }
-        "completed" => {
-            sqlx::query_as!(
-                TodoRow,
-                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
-                   created_at as "created_at!", updated_at as "updated_at!"
-                   FROM todos WHERE completed = 1 ORDER BY created_at DESC"#
-            )
-            .fetch_all(pool)
-            .await
-        }
-        _ => {
-            sqlx::query_as!(
-                TodoRow,
-                r#"SELECT id as "id!", title as "title!", completed as "completed!: bool",
-                   created_at as "created_at!", updated_at as "updated_at!"
-                   FROM todos ORDER BY created_at DESC"#
-            )
-            .fetch_all(pool)
-            .await
+    [sqlite] {
+        match filter {
+            "active" => sqlx::query_as!(TodoRow, r#"SELECT id as "id!", title as "title!", completed as "completed!: bool", created_at as "created_at!", updated_at as "updated_at!" FROM todos WHERE completed = 0 ORDER BY created_at DESC"#).fetch_all(pool).await,
+            "completed" => sqlx::query_as!(TodoRow, r#"SELECT id as "id!", title as "title!", completed as "completed!: bool", created_at as "created_at!", updated_at as "updated_at!" FROM todos WHERE completed = 1 ORDER BY created_at DESC"#).fetch_all(pool).await,
+            _ => sqlx::query_as!(TodoRow, r#"SELECT id as "id!", title as "title!", completed as "completed!: bool", created_at as "created_at!", updated_at as "updated_at!" FROM todos ORDER BY created_at DESC"#).fetch_all(pool).await,
         }
     }
 }

-// ---------------------------------------------------------------------------
-// todo_counts
-// ---------------------------------------------------------------------------
-
-#[cfg(feature = "postgres")]
-pub async fn todo_counts(pool: &DbPool) -> Result<(i64, i64), sqlx::Error> {
-    let counts = sqlx::query!(
-        r#"SELECT
-            COUNT(*) FILTER (WHERE NOT completed) as "active_count!",
-            COUNT(*) FILTER (WHERE completed) as "completed_count!"
-         FROM todos"#
-    )
-    .fetch_one(pool)
-    .await?;
-    Ok((counts.active_count, counts.completed_count))
-}
-
-#[cfg(feature = "sqlite")]
-pub async fn todo_counts(pool: &DbPool) -> Result<(i64, i64), sqlx::Error> {
-    let counts = sqlx::query!(
-        r#"SELECT
-            COALESCE(SUM(CASE WHEN NOT completed THEN 1 ELSE 0 END), 0) as "active_count!: i64",
-            COALESCE(SUM(CASE WHEN completed THEN 1 ELSE 0 END), 0) as "completed_count!: i64"
-         FROM todos"#
-    )
-    .fetch_one(pool)
-    .await?;
-    Ok((counts.active_count, counts.completed_count))
+dual_fn! {
+    pub async fn todo_counts(pool: &DbPool) -> Result<(i64, i64), sqlx::Error>
+    [postgres] {
+        let c = sqlx::query!(r#"SELECT COUNT(*) FILTER (WHERE NOT completed) as "active_count!", COUNT(*) FILTER (WHERE completed) as "completed_count!" FROM todos"#).fetch_one(pool).await?;
+        Ok((c.active_count, c.completed_count))
+    }
+    [sqlite] {
+        let c = sqlx::query!(r#"SELECT COALESCE(SUM(CASE WHEN NOT completed THEN 1 ELSE 0 END), 0) as "active_count!: i64", COALESCE(SUM(CASE WHEN completed THEN 1 ELSE 0 END), 0) as "completed_count!: i64" FROM todos"#).fetch_one(pool).await?;
+        Ok((c.active_count, c.completed_count))
+    }
 }

-// ---------------------------------------------------------------------------
-// insert_todo
-// ---------------------------------------------------------------------------
-
-#[cfg(feature = "postgres")]
-pub async fn insert_todo(pool: &DbPool, title: &str) -> Result<TodoRow, sqlx::Error> {
-    sqlx::query_as!(
-        TodoRow,
-        "INSERT INTO todos (title) VALUES ($1) RETURNING *",
-        title
-    )
-    .fetch_one(pool)
-    .await
-}
-
-#[cfg(feature = "sqlite")]
-pub async fn insert_todo(pool: &DbPool, title: &str) -> Result<TodoRow, sqlx::Error> {
-    let id = uuid::Uuid::new_v4().to_string();
-    sqlx::query_as!(
-        TodoRow,
-        r#"INSERT INTO todos (id, title) VALUES (?1, ?2)
-           RETURNING id as "id!", title as "title!", completed as "completed!: bool",
-           created_at as "created_at!", updated_at as "updated_at!""#,
-        id,
-        title
-    )
-    .fetch_one(pool)
-    .await
+dual_fn! {
+    pub async fn insert_todo(pool: &DbPool, title: &str) -> Result<TodoRow, sqlx::Error>
+    [postgres] {
+        sqlx::query_as!(TodoRow, "INSERT INTO todos (title) VALUES ($1) RETURNING *", title).fetch_one(pool).await
+    }
+    [sqlite] {
+        let id = uuid::Uuid::new_v4().to_string();
+        sqlx::query_as!(TodoRow, r#"INSERT INTO todos (id, title) VALUES (?1, ?2) RETURNING id as "id!", title as "title!", completed as "completed!: bool", created_at as "created_at!", updated_at as "updated_at!""#, id, title).fetch_one(pool).await
+    }
 }

-// ... same pattern for update_todo, delete_todo, count_active,
-//     set_all_completed, delete_completed ...
+dual_fn! {
+    pub async fn update_todo(pool: &DbPool, id: &TodoId, title: Option<&str>, completed: Option<bool>) -> Result<Option<TodoRow>, sqlx::Error>
+    [postgres] {
+        sqlx::query_as!(TodoRow, "UPDATE todos SET title = COALESCE($1, title), completed = COALESCE($2, completed), updated_at = now() WHERE id = $3 RETURNING *", title, completed, id).fetch_optional(pool).await
+    }
+    [sqlite] {
+        sqlx::query_as!(TodoRow, r#"UPDATE todos SET title = COALESCE(?1, title), completed = COALESCE(?2, completed), updated_at = datetime('now') WHERE id = ?3 RETURNING id as "id!", title as "title!", completed as "completed!: bool", created_at as "created_at!", updated_at as "updated_at!""#, title, completed, id).fetch_optional(pool).await
+    }
+}
+
+dual_fn! {
+    pub async fn delete_todo(pool: &DbPool, id: &TodoId) -> Result<bool, sqlx::Error>
+    [postgres] { Ok(sqlx::query!("DELETE FROM todos WHERE id = $1", id).execute(pool).await?.rows_affected() > 0) }
+    [sqlite]   { Ok(sqlx::query!("DELETE FROM todos WHERE id = ?1", id).execute(pool).await?.rows_affected() > 0) }
+}
+
+dual_fn! {
+    pub async fn count_active(pool: &DbPool) -> Result<i64, sqlx::Error>
+    [postgres] { Ok(sqlx::query!(r#"SELECT COUNT(*) as "cnt!" FROM todos WHERE NOT completed"#).fetch_one(pool).await?.cnt) }
+    [sqlite]   { Ok(sqlx::query!(r#"SELECT COUNT(*) as "cnt!: i64" FROM todos WHERE NOT completed"#).fetch_one(pool).await?.cnt) }
+}
+
+dual_fn! {
+    pub async fn set_all_completed(pool: &DbPool, completed: bool) -> Result<(), sqlx::Error>
+    [postgres] { sqlx::query!("UPDATE todos SET completed = $1, updated_at = now()", completed).execute(pool).await?; Ok(()) }
+    [sqlite]   { sqlx::query!("UPDATE todos SET completed = ?1, updated_at = datetime('now')", completed).execute(pool).await?; Ok(()) }
+}
+
+dual_fn! {
+    pub async fn delete_completed(pool: &DbPool) -> Result<(), sqlx::Error>
+    [postgres] { sqlx::query!("DELETE FROM todos WHERE completed = true").execute(pool).await?; Ok(()) }
+    [sqlite]   { sqlx::query!("DELETE FROM todos WHERE completed = 1").execute(pool).await?; Ok(()) }
+}

 #[cfg(feature = "test-helpers")]
 pub async fn delete_all(pool: &DbPool) -> Result<(), sqlx::Error> {
     sqlx::query!("DELETE FROM todos").execute(pool).await?;
     Ok(())
 }
```

## Decision

**What it buys you:** ~175 lines vs ~260, signatures can't drift between backends, no chance of a missing `#[cfg]`. `delete_all` stays standalone since it has shared SQL.

**What it costs:** IDE go-to-definition lands on the macro, not the function. `cargo sqlx prepare` still works (macros expand before sqlx's proc macros run). `rust-analyzer` may struggle with completions inside `dual_fn!` bodies. And the bodies are still fully duplicated — the macro only deduplicates the scaffolding around them.

**Net assessment: marginal win.** The current version is more grep-friendly and IDE-friendly for a modest amount of extra lines. We chose option 3 (accept it).
