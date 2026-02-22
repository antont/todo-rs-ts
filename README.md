# todo-rs-ts

Full-stack TodoMVC with a Rust API and a React frontend, connected by generated TypeScript types.

Demonstrates the pattern: define data types once in Rust, derive TypeScript interfaces with [ts-rs](https://github.com/Aleph-Alpha/ts-rs), and use them in the frontend — no manual type synchronization.

## Stack

| Layer | Tech |
|-------|------|
| API | [Axum](https://github.com/tokio-rs/axum) + [SQLx](https://github.com/launchbadge/sqlx) (direct queries, no ORM) |
| Database | PostgreSQL |
| Type bridge | [ts-rs](https://github.com/Aleph-Alpha/ts-rs) — Rust structs → TypeScript interfaces |
| Frontend | React 19, Vite, [TanStack Query](https://tanstack.com/query) |
| Migrations | [Refinery](https://github.com/rust-db/refinery) |

## Project structure

```
src/
  main.rs           # Axum server setup
  models.rs         # TodoRow (FromRow) + Todo/Request/Response (Serialize + TS)
  handlers.rs       # CRUD handlers with direct sqlx queries
  error.rs          # AppError → IntoResponse
  bin/migrate.rs    # Refinery migration runner
migrations/
  V1__todos.sql
web/
  src/
    api.ts          # Fetch client using generated types
    components/     # TodoApp, TodoItem, TodoFooter
    types/generated # ts-rs output (re-exported via index.ts)
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

This runs `cargo test` with `TS_RS_EXPORT_DIR` set, which writes `.ts` files to `web/src/types/generated/`.

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

**Separate DB and API types.** `TodoRow` uses `sqlx::FromRow` and maps 1:1 to the database schema (with native `Uuid` and `DateTime<Utc>`). `Todo` uses `Serialize + TS` and represents the JSON shape sent to the client (with `String` IDs and RFC 3339 timestamps). A `From<TodoRow>` impl bridges them. This separation means the database schema and API contract can evolve independently — adding a DB column doesn't force a frontend change until you're ready.

**Direct SQL, no ORM.** Each handler contains its own `sqlx::query_as` call with plain SQL. This makes it obvious what each endpoint does without navigating abstraction layers. For a small schema this is clearer than a query builder, and the pattern scales fine with a few dozen tables.

**Generated types as the contract.** The TypeScript types come from the Rust structs, not the other way around. If a Rust struct field changes, the frontend won't compile until types are regenerated. This catches integration mismatches at build time rather than runtime.

**TanStack Query, no client state library.** All todo data lives on the server. The frontend uses `useQuery` to fetch and `useMutation` + `invalidateQueries` to write. There's no reducer, no store, no synchronization logic — the server is the source of truth.

## How the type bridge works

Rust structs annotated with `#[derive(TS)]` and `#[ts(export)]` generate TypeScript interfaces when tests run:

```rust
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}
```

Produces:

```typescript
export type Todo = {
  id: string;
  title: string;
  completed: boolean;
  createdAt: string;
  updatedAt: string;
};
```

The frontend imports these types and uses them in the fetch client — if the Rust API shape changes, the TypeScript won't compile until the types are regenerated.

## License

MIT
