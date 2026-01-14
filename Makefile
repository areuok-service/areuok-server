.PHONY: help build run up down restart logs migrate test lint fmt-check install-hooks pre-commit

help:	## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Docker targets
build:	## Build Docker image
	docker build -t areuok-server .

run:	## Build and run with docker-compose
	docker-compose up --build

up:	## Start services with docker-compose
	docker-compose up -d

down:	## Stop and remove containers
	docker-compose down

restart:	## Restart services
	docker-compose restart

logs:	## Show logs from all services
	docker-compose logs -f

logs-server:	## Show server logs
	docker-compose logs -f server

logs-postgres:	## Show postgres logs
	docker-compose logs -f postgres

ps:	## List running containers
	docker-compose ps

clean:	## Stop and remove containers, networks, and volumes
	docker-compose down -v

# Testing targets
test:	## Run server tests
	cargo test --workspace

test-integration:	## Run integration tests with docker-compose
	docker-compose up -d
	sleep 5
	cargo test --workspace --test-threads=1
	docker-compose down

# Database targets
migrate:	## Run database migrations
	sqlx migrate run

migrate-docker:	## Run migrations in docker-compose
	docker-compose exec server sqlx migrate run

sqlx-prepare:	## Generate SQLx query data (requires running database)
	DATABASE_URL="postgresql://postgres:postgres@localhost:5432/areuok" cargo sqlx prepare --workspace

# Shell access
shell-server:	## Open shell in server container
	docker-compose exec server sh

shell-postgres:	## Open psql in postgres container
	docker-compose exec postgres psql -U postgres -d areuok

# Code quality targets
lint:	## Run all linters and checks
	@echo "🔍 Running format check..."
	cargo fmt --all -- --check
	@echo "🔎 Running Clippy..."
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "✅ All checks passed!"

fmt:	## Format code
	cargo fmt --all

fmt-check:	## Check code format
	cargo fmt --all -- --check

check:	## Check code compilation
	cargo check --workspace

clippy:	## Run clippy lints
	cargo clippy --workspace --all-targets -- -D warnings

clippy-fix:	## Run clippy and auto-fix issues
	cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

# Developer tools
install-hooks:	## Install pre-commit git hook
	@echo "📦 Installing pre-commit hook..."
	@chmod +x scripts/pre-commit
	@ln -sf ../../scripts/pre-commit .git/hooks/pre-commit
	@echo "✅ Pre-commit hook installed!"

pre-commit:	## Run pre-commit checks manually
	@./scripts/pre-commit

# Documentation
doc:	## Generate and open documentation
	cargo doc --workspace --no-deps --open

doc-check:	## Check documentation
	cargo doc --workspace --no-deps

# Build variants
dev:	## Build in debug mode
	cargo build --workspace

release:	## Build in release mode
	cargo build --workspace --release

# Clean
clean-build:	## Clean build artifacts
	cargo clean --workspace
