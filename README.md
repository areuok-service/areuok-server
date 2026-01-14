# areuok-server

A Rust-based server for device supervision and sign-in tracking system.

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Setup](#setup)
  - [Docker Setup](#docker-setup-recommended)
  - [Local Development Setup](#local-development-setup)
- [Running the Server](#running-the-server)
- [API Documentation](#api-documentation)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)
- [Database Schema](docs/database-schema.md)
- [Development Guide](#development-guide)
- [Deployment](#deployment)

## 🚀 Overview

AreUOK Server provides a comprehensive backend for device management, sign-in tracking, and supervision relationships. It's built with Rust and PostgreSQL, offering high performance and reliability.

## ✨ Features

- **Device Management**: Register and manage multiple devices with different modes
- **IMEI Binding**: Optional device binding via IMEI for device recovery and identification
- **Device Name Management**: Unique device names with 15-day update cooldown
- **Sign-in Tracking**: Track daily sign-ins with automatic streak counting
- **Supervision System**: Create supervision relationships between devices
- **RESTful API**: Clean and intuitive HTTP API
- **Database Migrations**: Automated schema management
- **Docker Support**: Containerized deployment for easy setup
- **Comprehensive Tests**: Full test coverage with pytest

## 🏗️ Architecture

This project uses a Cargo workspace structure:

```
areuok-server/
├── Cargo.toml              # Workspace configuration
├── Dockerfile              # Docker image definition
├── docker-compose.yml      # Docker services orchestration
├── crates/
│   ├── models/            # Data structures and types
│   ├── db/                # Database connection and migrations
│   ├── api/               # HTTP API handlers and routing
│   └── server/            # Main entry point and configuration
├── test/                  # API tests (Python/pytest)
└── docs/                  # Additional documentation
```

### Crate Descriptions

- **models**: Data structures (Device, SupervisionRequest, SigninRecord)
- **db**: Database connection pool, migrations, and query helpers
- **api**: HTTP handlers, routing, and error handling
- **server**: Application bootstrap, configuration, and main entry point

## 🎯 Quick Start

### Prerequisites

- Docker and Docker Compose
- OR Rust 1.70+ and PostgreSQL 12+ (for local development)

### Fastest Way to Start

```bash
# Clone and start everything in one command
git clone <repository-url>
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

## ⚙️ Setup

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
# Build the Docker image
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

- **PostgreSQL**: `localhost:5432