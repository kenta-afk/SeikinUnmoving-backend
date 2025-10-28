#!/bin/bash
set -e

echo "Starting database migrations..."

# Wait for postgres to be ready (additional check)
until pg_isready; do
  echo "Waiting for PostgreSQL to be ready..."
  sleep 2
done

echo "PostgreSQL is ready. Running migrations..."

# Run all SQL files in /migrations directory in alphabetical order
for file in /migrations/*.sql; do
  if [ -f "$file" ]; then
    echo "Running migration: $file"
    psql -v ON_ERROR_STOP=1 < "$file"
    echo "✓ Completed: $file"
  fi
done

echo "All migrations completed successfully!"
exit 0
