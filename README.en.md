# areuok-server - Cloud Server

English | [简体中文](./README.md)

A high-performance backend server for device management and sign-in tracking system, built with Rust and PostgreSQL.

> 🎯 **areuok-server** is the companion cloud service for the areuok client application, providing device management, multi-device supervision, sign-in data synchronization, and streak calculation features.

## Features

- 📱 **Device Management** - Register and manage multiple devices with different modes
- 🔐 **IMEI Binding** - Optional device IMEI binding for device recovery and identification
- 🏷️ **Device Name Management** - Globally unique device names with 15-day update cooldown
- 🔥 **Sign-in Tracking** - Track daily sign-ins with automatic consecutive day counting
- 👀 **Supervision System** - Create supervision relationships between devices
- 🌐 **RESTful API** - Clean and intuitive HTTP API
- 🗄️ **Database Migrations** - Automated schema management
- 🐳 **Docker Support** - Containerized deployment for quick and easy setup
- ✅ **Comprehensive Tests** - Full test coverage with pytest

## Architecture

This project uses a Cargo workspace structure:

```
┌─────────────────────────────────────────────────────────────┐
│                    areuok Server                           │
│              (Rust + Axum + PostgreSQL)                     │
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │   models    │    │     db      │    │    api      │     │
│  │Data Types   │    │   Database   │    │ HTTP Routes │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐   │
│  │                   server                             │   │
│  │             App Entry & Configuration                  │   │
│  └───────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
                   PostgreSQL Database
```

### Crate Descriptions

- **models** - Data structures (Device, SupervisionRequest, SigninRecord)
- **db** - Database connection pool, migrations, and query helpers
- **api** - HTTP handlers, routing, and error handling
- **server** - Application bootstrap, configuration, and main entry point

## Quick Start

### Prerequisites

- Docker and Docker Compose
- OR Rust 1.70+ and PostgreSQL 12+ (for local development)

### Fastest Way to Start

```bash
# Clone and start everything in one command
git clone https://github.com/nicepeng/areuok-server.git
cd areuok-server
./start-docker.sh
```

The server will be available at `http://localhost:3000`

### Quick Test

```bash
# Test device registration
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{"device_name": "my-device", "mode": "signin"}'

# Run all API tests
cd test
uv run pytest -v
```

## Setup

### Docker Setup (Recommended)

#### Option 1: Using Start Script

```bash
./start-docker.sh
```

This script:
1. Builds the Docker image
2. Starts PostgreSQL and server containers
3. Shows logs

#### Option 2: Using Makefile

```bash
# Start all services (builds Docker image if needed)
make up

# Start and build from scratch
make run

# View logs
make logs

# View server logs only
make logs-server

# Stop services
make down

# Stop and remove all data (including database)
make clean

# Build Docker image only
make build

# Run tests
make test
```

#### Option 3: Manual Docker Commands

```bash
# Build Docker image
docker build -t areuok-server .

# Start all services (PostgreSQL + Server)
docker-compose up -d

# View logs
docker-compose logs -f server

# Stop services
docker-compose down

# Stop and remove volumes (clears database data)
docker-compose down -v

# Restart services
docker-compose restart server
```

#### Docker Services

- **PostgreSQL**: `localhost:5432`
  - Username: `postgres`
  - Password: `postgres`
  - Database: `areuok`
- **Server**: `localhost:3000`

### Local Development Setup

#### 1. Install Dependencies

```bash
# Clone repository
git clone https://github.com/nicepeng/areuok-server.git
cd areuok-server

# Install Rust dependencies
cargo build
```

#### 2. Configure Database

Ensure PostgreSQL 12+ is installed and create the database:

```bash
# Create database
createdb areuok

# Configure environment variables
cp .env.example .env
# Edit .env file and set DATABASE_URL
```

#### 3. Run Migrations

```bash
# Run migrations before starting the server
make migrate

# Or using cargo
cargo run -- migrate
```

#### 4. Start Server

```bash
# Development mode
cargo run

# Or using make
make run
```

Server will start on `http://localhost:3000`

### Database Migrations

```bash
# Run migrations (requires running server)
make migrate

# Run migrations in Docker
make migrate-docker
```

## API Documentation

### Device Management

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/devices/register` | POST | Register new device |
| `/devices/{id}` | GET | Get device information |
| `/search/devices?q={query}` | GET | Search devices (min 2 characters) |
| `/devices/{id}/signin` | POST | Device sign-in |
| `/devices/{id}/status` | GET | Get sign-in status |

### Device Registration

```bash
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "my-device",
    "mode": "signin",
    "imei": "optional-imei"
  }'
```

Response:
```json
{
  "device_id": "uuid",
  "device_name": "my-device",
  "mode": "signin",
  "imei": "optional-imei",
  "created_at": "2025-01-15T10:00:00Z"
}
```

### Device Sign-in

```bash
curl -X POST http://localhost:3000/devices/{device_id}/signin
```

Response:
```json
{
  "device_id": "uuid",
  "signin_date": "2025-01-15",
  "streak": 5,
  "created_at": "2025-01-15T10:00:00Z"
}
```

### Get Sign-in Status

```bash
curl http://localhost:3000/devices/{device_id}/status
```

Response:
```json
{
  "device_id": "uuid",
  "device_name": "my-device",
  "mode": "signin",
  "current_streak": 5,
  "last_signin_date": "2025-01-15",
  "today_signed_in": true
}
```

### Supervision Management

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/supervision/request` | POST | Create supervision request |
| `/supervision/pending/{id}` | GET | Get pending supervision requests |
| `/supervision/accept` | POST | Accept supervision request |
| `/supervision/reject` | POST | Reject supervision request |
| `/supervision/list/{id}` | GET | Get supervision relationships list |
| `/supervision/{relation_id}` | DELETE | Remove supervision relationship |

### Create Supervision Request

```bash
curl -X POST http://localhost:3000/supervision/request \
  -H "Content-Type: application/json" \
  -d '{
    "requester_id": "device-uuid-1",
    "target_id": "device-uuid-2"
  }'
```

### Accept Supervision Request

```bash
curl -X POST http://localhost:3000/supervision/accept \
  -H "Content-Type: application/json" \
  -d '{
    "request_id": "request-uuid"
  }'
```

### Get Supervision Relationships

```bash
curl http://localhost:3000/supervision/list/{device_id}
```

Response:
```json
{
  "supervising": [
    {
      "relation_id": "uuid",
      "target_device_name": "device-name",
      "target_device_id": "uuid",
      "created_at": "2025-01-15T10:00:00Z"
    }
  ],
  "supervised_by": [
    {
      "relation_id": "uuid",
      "supervisor_device_name": "supervisor-name",
      "supervisor_device_id": "uuid",
      "created_at": "2025-01-15T10:00:00Z"
    }
  ]
}
```

## Testing

### Rust Tests

```bash
# Run all Rust tests
cargo test

# Using make
make test
```

### Integration Tests

```bash
# Run integration tests with Docker Compose
make test-integration
```

### API Tests

```bash
cd test

# Install Python dependencies
uv pip install -r requirements.txt

# Run all tests
uv run pytest -v

# Run specific tests
uv run pytest -v -k "test_device"

# View test coverage
uv run pytest --cov=. --cov-report=html
```

## Development

### Requirements

- Rust 1.70+
- PostgreSQL 12+
- Docker and Docker Compose (recommended)

### Code Check

```bash
# Quick compile check
cargo check

# Build project
cargo build

# Run server
cargo run
```

### Code Quality

```bash
# Format code
make fmt

# Run Clippy linter
make clippy

# Or using cargo
cargo fmt
cargo clippy -- -D warnings
```

### Database Operations

```bash
# Run migrations
make migrate

# View database schema
cat crates/db/migrations/*.sql
```

## Environment Variables

| Variable | Required | Description | Default |
|----------|-----------|-------------|---------|
| `DATABASE_URL` | Yes | PostgreSQL connection string | - |
| `RUST_LOG` | No | Log level | `info,server=debug,api=debug,db=debug` |

### Environment Configuration Example

```bash
# .env file example
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/areuok
RUST_LOG=info,server=debug,api=debug,db=debug
```

## Tech Stack

- **Language**: Rust 1.70+
- **Web Framework**: Axum 0.7
- **Database**: PostgreSQL 16
- **Database Driver**: SQLx 0.8
- **Async Runtime**: Tokio 1.49
- **Serialization**: Serde 1.0
- **Containerization**: Docker & Docker Compose
- **Testing**: pytest (Python API tests)

## Database Schema

Database schema is located in `crates/db/migrations/` directory and includes the following tables:

- `devices` - Device information
- `signin_records` - Sign-in records
- `supervision_requests` - Supervision requests
- `supervision_relations` - Supervision relationships

For detailed database schema documentation, see [docs/database-schema.md](docs/database-schema.md)

## Project Structure

```
areuok-server/
├── Cargo.toml              # Workspace configuration
├── Dockerfile              # Docker image definition
├── docker-compose.yml      # Docker services orchestration
├── start-docker.sh        # Quick start script
├── Makefile               # Common commands
├── .env.example           # Environment variables example
├── crates/
│   ├── models/            # Data structures and types
│   ├── db/                # Database connection and migrations
│   ├── api/               # HTTP API handlers and routing
│   └── server/            # Main entry point and configuration
├── test/                  # API tests (Python/pytest)
│   ├── conftest.py       # Test configuration
│   ├── test_device.py    # Device-related tests
│   ├── test_signin.py    # Sign-in-related tests
│   └── test_supervision.py # Supervision-related tests
└── docs/                  # Additional documentation
    ├── database-schema.md
    └── lint-setup.md
```

## Deployment

### Docker Deployment

For production deployment, it's recommended to use Docker Compose:

```bash
# Build production image
docker build -t areuok-server:latest .

# Start with production configuration
docker-compose -f docker-compose.yml up -d

# View logs
docker-compose logs -f
```

### Production Considerations

1. **Database Security**
   - Change default PostgreSQL password
   - Use strong passwords
   - Restrict database access by IP

2. **HTTPS Configuration**
   - Use reverse proxy (e.g., Nginx)
   - Configure SSL certificates

3. **Log Management**
   - Configure appropriate log levels
   - Set up log rotation

4. **Monitoring**
   - Add health check endpoints
   - Monitor server performance

## Troubleshooting

### Common Issues

**1. Database Connection Failed**

```bash
# Check if PostgreSQL is running
docker ps | grep postgres

# Check database connection string
echo $DATABASE_URL

# Restart database
docker-compose restart postgres
```

**2. Port Already in Use**

```bash
# Check port usage
lsof -i :3000
lsof -i :5432

# Modify port mapping in docker-compose.yml
```

**3. Migration Failed**

```bash
# Check migration files
ls -la crates/db/migrations/

# View migration logs
docker-compose logs postgres

# Re-run migrations
docker-compose down -v
docker-compose up -d
```

**4. Tests Failed**

```bash
# Ensure service is running
curl http://localhost:3000

# View server logs
docker-compose logs server

# Clean and restart
docker-compose down
docker-compose up -d
```

## Recommended IDE

[VS Code](https://code.visualstudio.com/) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) + [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

## Documentation

- [Database Schema Documentation](docs/database-schema.md)
- [Lint Setup Guide](docs/lint-setup.md)
- [Contributing Guide](CONTRIBUTING.md)
- [License](LICENSE)

## Related Repositories

| Repository | Description |
|------------|-------------|
| [areuok](https://github.com/nicepeng/areuok) | 📱 Client application (Tauri + SvelteKit) |
| [areuok-server](https://github.com/nicepeng/areuok-server) | ☁️ Cloud server (this repository) |

## License

This project is licensed under the [GNU General Public License v2.0 (GPLv2)](./LICENSE).

## Acknowledgements

### Tech Stack Credits

This project wouldn't be possible without these amazing open-source projects:

**Core Frameworks**
- [Rust](https://www.rust-lang.org/) - A language empowering everyone to build reliable and efficient software
- [Axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio
- [Tokio](https://tokio.rs/) - Rust's asynchronous runtime

**Database**
- [PostgreSQL](https://www.postgresql.org/) - The world's most advanced open source relational database
- [SQLx](https://github.com/launchbadge/sqlx) - The Rust SQL toolkit

**Utility Libraries**
- [Serde](https://serde.rs/) - Serialization/deserialization framework for Rust
- [Chrono](https://github.com/chronotope/chrono) - Date and time library for Rust
- [UUID](https://github.com/uuid-rs/uuid) - UUID generation and parsing

**DevOps**
- [Docker](https://www.docker.com/) - Containerization platform
- [Docker Compose](https://docs.docker.com/compose/) - Multi-container Docker applications

Thanks to all open-source contributors for their hard work!

## Contributing

Issues and Pull Requests are welcome! Please read the [Contributing Guide](./CONTRIBUTING.md) first.
