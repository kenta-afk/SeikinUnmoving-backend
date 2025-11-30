#!/bin/bash
set -e

echo "Starting database migration..."
echo "DATABASE_URL: ${DATABASE_URL}"

# Create database directory if it doesn't exist
mkdir -p /data

# Create empty database file if it doesn't exist
touch /data/seikin.db

# Set proper permissions
chmod 666 /data/seikin.db

# Wait for database file to be accessible
sleep 1

# Change to app directory where migrations are located
cd /app

# Run migrations with explicit source directory
sqlx migrate run --source /app/db/migrations --database-url "${DATABASE_URL}"

echo "Migration completed successfully!"

# Start the application in debug mode for development
echo "Starting apiroute server in development mode..."
exec /app/target/debug/apiroute
