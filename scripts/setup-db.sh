#!/usr/bin/env bash
set -euo pipefail

DB_URL="${DATABASE_URL:-postgres://postgres@localhost/todo_app}"
DB_NAME=$(echo "$DB_URL" | grep -oE '[^/]+$')

echo "Creating database '$DB_NAME' (if it doesn't exist)..."
createdb -U postgres "$DB_NAME" 2>/dev/null || true

echo "Running migrations..."
DATABASE_URL="$DB_URL" cargo run --bin todo-migrate

# Test database
TEST_DB_URL="${TEST_DATABASE_URL:-postgres://postgres@localhost/todo_app_test}"
TEST_DB_NAME=$(echo "$TEST_DB_URL" | grep -oE '[^/]+$')

echo "Creating test database '$TEST_DB_NAME' (if it doesn't exist)..."
createdb -U postgres "$TEST_DB_NAME" 2>/dev/null || true

echo "Running migrations on test database..."
DATABASE_URL="$TEST_DB_URL" cargo run --bin todo-migrate

echo "Done."
