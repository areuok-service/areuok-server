#!/bin/bash

set -e

echo "=========================================="
echo "   AreuOK Server Quick Start"
echo "=========================================="
echo ""

# Check if docker-compose is installed
if ! command -v docker-compose &> /dev/null; then
    if command -v docker &> /dev/null; then
        echo "Docker Compose not found, but Docker is installed."
        echo "You may need to use 'docker compose' instead of 'docker-compose'"
    else
        echo "Error: Neither Docker nor Docker Compose is installed."
        echo "Please install Docker first: https://docs.docker.com/get-docker/"
        exit 1
    fi
fi

# Start services
echo "Starting PostgreSQL and server..."
docker-compose up -d --build

echo ""
echo "=========================================="
echo "   Services started!"
echo "=========================================="
echo ""
echo "PostgreSQL:"
echo "  - Host: localhost"
echo "  - Port: 5432"
echo "  - User: postgres"
echo "  - Password: postgres"
echo "  - Database: areuok"
echo ""
echo "Server:"
echo "  - URL: http://localhost:3000"
echo ""
echo "To stop services:"
echo "  make down"
echo "  docker-compose down"
echo ""
echo "To view logs:"
echo "  make logs"
echo "  docker-compose logs -f"
echo ""
echo "To run migrations manually:"
echo "  make migrate"
echo ""
echo "=========================================="
