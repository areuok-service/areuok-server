# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

### Docker (Recommended)
```bash
make up           # Start all services (PostgreSQL + server)
make run          # Build and start with docker-compose
make down         # Stop services
make logs         # View logs from all services
make logs-server  # View server logs only
make clean        # Stop and remove all containers, networks, and volumes
```

### Local Development
```bash
cargo check       # Verify code compiles (faster than build)
cargo build       # Build the project
cargo run         # Run the server (requires DATABASE_URL set)
cargo test        # Run Rust tests
```

### Database
```bash
make migrate           # Run migrations (requires running server)
make migrate-docker    # Run migrations in docker-compose
```

### Code Quality
```bash
make fmt        # Format code with rustfmt
make clippy     # Run linter with strict warnings
```

### Testing
```bash
make test               # Run Rust tests
make test-integration   # Run integration tests with docker-compose

cd test && uv run pytest -v              # Run Python API tests
cd test && uv run pytest -v -k "test_device"  # Run specific test subset
```

## Architecture

This is a Cargo workspace with 4 crates:

### Core Architecture Flow
`server` → `api` → `db` → PostgreSQL
              ↓
           `models`

### Crate Responsibilities

**models** - Data types only
- Defines all domain types: `Device`, `SupervisionRequest`, `SigninRecord`, etc.
- Enums for `DeviceMode` (signin/supervisor) and `SupervisionStatus` (pending/accepted/rejected)
- No dependencies on other crates

**db** - Database connectivity
- `create_pool()` - Creates PostgreSQL connection pool (reads `DATABASE_URL` env var)
- `run_migrations()` - Runs SQL migrations from `./migrations/` directory
- Max 10 connections configured in pool

**api** - HTTP layer
- `create_router(pool)` - Returns Axum router with all routes
- Routes are organized in `lib.rs` with handlers split into modules:
  - `handlers/signin.rs` - Sign-in logic with streak calculation
  - `handlers/supervision.rs` - Supervision request/accept/reject/list/remove
- Error handling via `error.rs` - converts to HTTP status codes and JSON responses
- All handlers use `sqlx::query!` and `sqlx::query_as!` macros with type-safe queries

**server** - Application entry point
- `main.rs` bootstraps the application:
  1. Loads `.env` file
  2. Initializes env_logger (default filter: `info,server=debug,api=debug,db=debug`)
  3. Creates database pool
  4. Runs migrations
  5. Creates router with CORS and tracing layers
  6. Binds to port 3000

### Key Implementation Details

**Streak Calculation** (signin.rs:61-69)
- Checks if yesterday's signin exists for the device
- If yes: increments streak by 1
- If no: resets to 1
- Returns existing record if already signed in today

**Supervision Request Flow** (supervision.rs)
1. `create_supervision_request` - Creates pending request
2. `pending_requests` - Lists pending requests for a device
3. `accept_supervision` - Updates request to 'accepted' AND creates relation in `supervision_relations` table
4. `reject_supervision` - Updates request to 'rejected'
5. `list_supervision_relations` - Lists all relations for a device (both as supervisor and target)
6. `remove_supervision` - Deletes relation by ID

**SQLx Type Safety**
- Custom types require explicit casting: `mode as "mode: models::DeviceMode"`
- This enables compile-time query verification when `DATABASE_URL` is set
- Migrations in `crates/db/migrations/` define database schema

**Device Search** (lib.rs:165-192)
- `GET /search/devices?q=<query>` endpoint
- Requires minimum 2 characters
- ILIKE search on device_name with pattern matching
- Returns max 20 results ordered by device_name

## Environment

Required environment variable:
- `DATABASE_URL` - PostgreSQL connection string (format: `postgresql://user:pass@host:port/db`)
  - Docker: See `.env.example` and `docker-compose.yml`
  - Local: Create `.env` file with database URL

Optional:
- `RUST_LOG` - Logging level (default: `info,server=debug,api=debug,db=debug`)

## API Endpoints

```
POST   /devices/register
GET    /devices/:id
GET    /search/devices?q=<query>
POST   /devices/:id/signin
GET    /devices/:id/status

POST   /supervision/request
GET    /supervision/pending/:id
POST   /supervision/accept
POST   /supervision/reject
GET    /supervision/list/:id
DELETE /supervision/:relation_id
```

Server runs on port 3000.
