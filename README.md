# todo-rs-ts

Full-stack TodoMVC with a Rust API and a React frontend, connected by generated TypeScript types.

Demonstrates the pattern: define data types once in Rust, generate TypeScript interfaces with [typeshare](https://github.com/1Password/typeshare), and use them in the frontend — no manual type synchronization.

## Stack

| Layer | Tech |
|-------|------|
| API | [Axum](https://github.com/tokio-rs/axum) + [SQLx](https://github.com/launchbadge/sqlx) (direct queries, no ORM) |
| Database | PostgreSQL |
| Type bridge | [typeshare](https://github.com/1Password/typeshare) — Rust structs → TypeScript interfaces |
| Frontend | React 19, Vite, [TanStack Query](https://tanstack.com/query) |
| Migrations | [SQLx](https://github.com/launchbadge/sqlx) built-in (`sqlx::migrate!`) |

## Project structure

```
src/
  main.rs           # Axum server setup
  lib.rs            # Shared library root
  models.rs         # TodoRow (FromRow) + Todo/Request/Response (Serialize + typeshare)
  handlers.rs       # CRUD handlers
  queries.rs        # Direct sqlx queries (cfg-gated postgres/sqlite variants)
  error.rs          # AppError → IntoResponse
  bin/migrate.rs    # SQLx migration runner (postgres)
migrations/
  0001_todos.sql
web/
  src/
    api.ts          # Fetch client using generated types
    components/     # TodoApp, TodoItem, TodoFooter
    types/generated # typeshare output (re-exported via index.ts)
scripts/
  setup-db.sh       # Create database + run migrations
  generate-types.sh # Generate TypeScript from Rust structs
```

## Getting started

### Prerequisites

- Rust (stable)
- Node.js 18+
- PostgreSQL running locally

### 1. Set up the database

```bash
cp .env.example .env
# Edit .env if your Postgres connection differs from the default

bash scripts/setup-db.sh
```

### 2. Generate TypeScript types

```bash
bash scripts/generate-types.sh
```

This runs the `typeshare` CLI, which parses Rust source files and writes TypeScript interfaces to `web/src/types/generated/types.ts`.

### 3. Start the API

```bash
source .env
cargo run --bin todo-api
# Listening on http://0.0.0.0:3001
```

### 4. Start the frontend

```bash
cd web
npm install
npm run dev
# http://localhost:5173
```

## API

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/todos?filter=all\|active\|completed` | List todos |
| `POST` | `/api/todos` | Create a todo |
| `PATCH` | `/api/todos/{id}` | Update title and/or completed |
| `DELETE` | `/api/todos/{id}` | Delete a todo |
| `POST` | `/api/todos/toggle-all` | Toggle all todos |
| `DELETE` | `/api/todos/completed` | Clear completed todos |
| `DELETE` | `/api/test/cleanup` | Delete all todos (`test-helpers` feature only) |

## Testing

Integration tests live in `web/tests/` and hit the API over HTTP using the same generated types the frontend uses. They run with [Vitest](https://vitest.dev/).

```bash
# Start the API against the test database with test-helpers enabled (terminal 1)
DATABASE_URL=postgres://localhost/todo_app_test cargo run --features test-helpers --bin todo-api

# Run tests (terminal 2)
cd web && npm test
```

The `test-helpers` Cargo feature flag enables `DELETE /api/test/cleanup`, which tests use to clear data before each test case. This endpoint is not compiled into the binary without the feature flag.

## Design decisions

**Separate DB and API types.** `TodoRow` uses `sqlx::FromRow` and maps 1:1 to the database schema (with native `Uuid` and `DateTime<Utc>`). `Todo` uses `Serialize` + `#[typeshare]` and represents the JSON shape sent to the client (with `String` IDs and RFC 3339 timestamps). A `From<TodoRow>` impl bridges them. This separation means the database schema and API contract can evolve independently — adding a DB column doesn't force a frontend change until you're ready.

**Direct SQL, no ORM.** All SQL lives in `src/queries.rs` as compile-time verified `sqlx::query!` calls, with cfg-gated postgres and sqlite variants. Handlers in `src/handlers.rs` are cfg-free and only deal with HTTP/validation logic. The postgres and sqlite query functions are intentionally duplicated rather than abstracted with a macro — see [docs/query-layer-duplication-rationale.md](docs/query-layer-duplication-rationale.md) for the analysis and tradeoffs.

**Generated types as the contract.** The TypeScript types come from the Rust structs, not the other way around. If a Rust struct field changes, the frontend won't compile until types are regenerated. This catches integration mismatches at build time rather than runtime.

**TanStack Query, no client state library.** All todo data lives on the server. The frontend uses `useQuery` to fetch and `useMutation` + `invalidateQueries` to write. There's no reducer, no store, no synchronization logic — the server is the source of truth.

## How the type bridge works

Rust structs annotated with `#[typeshare]` generate TypeScript interfaces via the [typeshare CLI](https://github.com/1Password/typeshare):

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

Running `typeshare ./src --lang=typescript --output-file=web/src/types/generated/types.ts` produces:

```typescript
export interface Todo {
  id: string;
  title: string;
  completed: boolean;
  createdAt: string;
  updatedAt: string;
}
```

The frontend imports these types and uses them in the fetch client — if the Rust API shape changes, the TypeScript won't compile until the types are regenerated.

## License

MIT
