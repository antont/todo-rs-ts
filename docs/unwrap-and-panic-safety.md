# Unwrap and Panic Safety

## The rule

Never call `.unwrap()` in code that runs per-request. Use `?` with proper error types instead.

A panic in a handler crashes the connection (or the entire server if not using a panic-catching layer). Returning `Result<_, AppError>` gives callers a proper HTTP error response with the right status code — no crash, no dropped connection.

```rust
// Bad: panics on unexpected None/Err
let value = something.unwrap();

// Good: propagates error as HTTP response
let value = something?;
let value = something.map_err(|e| AppError::Internal(e.to_string()))?;
```

## Startup is different

`expect("message")` is correct for configuration, database connection, and bind operations at startup. If the server can't start, it should fail fast with a clear message — not limp along in a broken state.

```rust
let pool = PgPool::connect(&url).await.expect("Failed to connect to database");
let listener = TcpListener::bind(addr).await.expect("Failed to bind");
```

## Tests are exempt

`.unwrap()` in tests is idiomatic Rust. A panic gives a backtrace pointing directly at the failure, which is exactly what you want in a test.

## The Clippy lint

`clippy::unwrap_used` flags any call to `.unwrap()` or `.expect()` on `Option`/`Result`. When set to `deny`, it becomes a compile error.

This project uses `#![deny(clippy::unwrap_used)]` as an inner attribute at the top of modules where unwrap would be dangerous:

```rust
// At the top of src/handlers.rs, src/queries.rs, src/error.rs
#![deny(clippy::unwrap_used)]
```

If you add `.unwrap()` to one of these files, `cargo clippy` will reject it. Use `?` instead, or `unwrap_or` / `unwrap_or_else` for safe defaults.

## Why this matters for HTTP servers

| Approach | What happens on bad input |
|----------|--------------------------|
| `.unwrap()` | Panic. Connection dropped. Possible server crash. |
| `?` + `AppError` | `400 Bad Request` or `500 Internal Server Error` with JSON body. Server stays up. |

Axum does not install a panic-catching layer by default. A panic in a handler propagates up and kills the task — the client gets a connection reset with no useful error message.

## This project's convention

| Location | Rule |
|----------|------|
| `src/handlers.rs` | `#![deny(clippy::unwrap_used)]` — use `?` with `AppError` |
| `src/queries.rs` | `#![deny(clippy::unwrap_used)]` — use `?` to propagate `sqlx::Error` |
| `src/error.rs` | `#![deny(clippy::unwrap_used)]` — error conversions must not panic |
| `src/main.rs` | `.expect("message")` for startup; no lint (startup panics are correct) |
| `src/models.rs` | No lint needed — type definitions only, no unwrap present |
| `tests/` | `.unwrap()` is fine — idiomatic for tests |

`unwrap_or` and `unwrap_or_else` are always acceptable — they cannot panic.
