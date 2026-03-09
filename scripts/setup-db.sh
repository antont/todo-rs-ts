#!/usr/bin/env bash
set -euo pipefail

DB_URL="${DATABASE_URL:-postgres://postgres@db/todo_app}"
DB_NAME=$(echo "$DB_URL" | grep -oE '[^/]+$')
# Derive a connection URL pointing to the default 'postgres' database for createdb
DB_BASE_URL=$(echo "$DB_URL" | sed "s|/$DB_NAME\$|/postgres|")

echo "Creating database '$DB_NAME' (if it doesn't exist)..."
psql "$DB_BASE_URL" -tc "SELECT 1 FROM pg_database WHERE datname = '$DB_NAME'" | grep -q 1 || psql "$DB_BASE_URL" -c "CREATE DATABASE $DB_NAME"

echo "Running migrations..."
DATABASE_URL="$DB_URL" sqlx migrate run --source migrations/

# Test database
TEST_DB_URL="${TEST_DATABASE_URL:-postgres://postgres@db/todo_app_test}"
TEST_DB_NAME=$(echo "$TEST_DB_URL" | grep -oE '[^/]+$')

echo "Creating test database '$TEST_DB_NAME' (if it doesn't exist)..."
psql "$DB_BASE_URL" -tc "SELECT 1 FROM pg_database WHERE datname = '$TEST_DB_NAME'" | grep -q 1 || psql "$DB_BASE_URL" -c "CREATE DATABASE $TEST_DB_NAME"

echo "Running migrations on test database..."
DATABASE_URL="$TEST_DB_URL" sqlx migrate run --source migrations/

echo "Done."
