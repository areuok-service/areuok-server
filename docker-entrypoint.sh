#!/bin/sh

set -e

echo "Waiting for PostgreSQL to start..."
until pg_isready -h postgres -U postgres -p 5432; do
  echo "PostgreSQL is unavailable - sleeping"
  sleep 1
done

echo "PostgreSQL is up - running migrations"
sqlx migrate run --database-url postgresql://postgres:postgres@postgres:5432/areuok || {
    echo "Migration failed, but continuing..."
}

echo "Starting server..."
exec "$@"
