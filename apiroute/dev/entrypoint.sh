#!/bin/bash
set -e

echo "Starting database migration..."

# Wait for database file to be accessible
sleep 1

# Run migrations
sqlx migrate run --database-url "${DATABASE_URL}"

echo "Migration completed successfully!"

# Start the application
echo "Starting apiroute server..."
exec /app/apiroute
