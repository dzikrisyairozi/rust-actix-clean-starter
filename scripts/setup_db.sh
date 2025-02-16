#!/bin/bash
set -e

# Load environment variables
source .env

echo "Creating database..."
sqlx database create

echo "Running initialization script..."
# Use psql instead of sqlx query
PGPASSWORD=$POSTGRES_PASSWORD psql -h localhost -U $POSTGRES_USER -d $POSTGRES_DB -f src/infrastructure/database/init.sql

echo "Verifying database setup..."
PGPASSWORD=$POSTGRES_PASSWORD psql -h localhost -U $POSTGRES_USER -d $POSTGRES_DB -c "SELECT COUNT(*) FROM users;"
PGPASSWORD=$POSTGRES_PASSWORD psql -h localhost -U $POSTGRES_USER -d $POSTGRES_DB -c "SELECT COUNT(*) FROM products;"

echo "Database setup completed successfully!"