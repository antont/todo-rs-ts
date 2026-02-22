# AGENTS.md

Instructions for AI coding agents working on this project.

## Build & run

```bash
# Rust — check compilation
cargo check

# Rust — run API server (requires DATABASE_URL)
source .env
cargo run --bin todo-api

# Rust — run migrations
DATABASE_URL=postgres://localhost/todo_app cargo run --bin todo-migrate

# Frontend — install deps (from web/)
cd web && npm install

# Frontend — dev server
cd web && npm run dev

# Frontend — type check
cd web && npx tsc --noEmit

# Frontend — production build
cd web && npx vite build

# Integration tests (requires API running against test DB with test-helpers feature)
# Terminal 1:
DATABASE_URL=postgres://localhost/todo_app_test cargo run --features test-helpers --bin todo-api
# Terminal 2:
cd web && npm test
```

## Type generation

TypeScript types are generated from Rust structs via ts-rs. After changing any struct with `#[ts(export)]` in `src/models.rs`:

```bash
TS_RS_EXPORT_DIR=web/src/types/generated cargo test
```

This writes individual `.ts` files to `web/src/types/generated/`. The barrel `index.ts` in that directory re-exports them — update it if you add or remove types.

## Architecture

- **Single Rust crate, two binaries**: `todo-api` (server) and `todo-migrate` (migration runner).
- **No ORM**: handlers use `sqlx::query_as` with raw SQL. Keep it that way.
- **Separate DB and API types**: `TodoRow` (with `FromRow`) is the database row; `Todo` (with `Serialize + TS`) is the API response. Convert with `From<TodoRow>`. Don't merge them.
- **Frontend state**: TanStack Query manages server state. Mutations invalidate the `['todos']` query key. No client-side state management library.

## Conventions

- Rust structs for API responses use `#[serde(rename_all = "camelCase")]` — all JSON is camelCase.
- API handlers return `Result<Json<T>, AppError>`. Add new error variants to `src/error.rs` as needed.
- SQL migrations go in `migrations/` with refinery naming: `V{N}__{description}.sql`.
- Frontend components are in `web/src/components/`. One component per file, named export matching filename.

## Common tasks

**Add a new field to todos:**
1. Add column in a new migration `migrations/V2__description.sql`
2. Add field to `TodoRow` in `src/models.rs`
3. Add field to `Todo` (and update `From<TodoRow>`)
4. Regenerate types: `TS_RS_EXPORT_DIR=web/src/types/generated cargo test`
5. Update frontend components to use the new field

**Add a new endpoint:**
1. Add handler function in `src/handlers.rs`
2. Add route in `src/main.rs`
3. Add fetch function in `web/src/api.ts`
4. If new request/response types are needed, add to `src/models.rs` with `#[ts(export)]`, regenerate, and update `web/src/types/generated/index.ts`

## Testing

Integration tests are in `web/tests/`. They hit the real API over HTTP using the generated TypeScript types — proving the type bridge works end-to-end.

- Tests assume the API is running on `localhost:3001` (override with `API_URL` env var)
- The API must be started with `--features test-helpers` — this enables `DELETE /api/test/cleanup` used by `beforeEach` to clear data
- Each test clears all data via `DELETE /api/test/cleanup` in `beforeEach`
- Use a separate test database (`todo_app_test`) to avoid clobbering dev data
- `scripts/setup-db.sh` creates and migrates both dev and test databases

**Add a new test:** add a `test()` block in `web/tests/todos.test.ts`. Use the helpers from `web/tests/helpers.ts` (`createTodo`, `listTodos`, `api`, `apiStatus`, `clearTodos`).

## Gotchas

- ts-rs maps Rust `i64` to TypeScript `bigint` by default. Use `#[ts(type = "number")]` for JSON-serialized integer fields.
- ts-rs maps `Option<T>` to `T | null`, not `T | undefined`. The frontend `api.ts` converts `undefined` → `null` when building request bodies.
- The API CORS config only allows `http://localhost:5173`. Update `src/main.rs` if the frontend runs on a different origin.
