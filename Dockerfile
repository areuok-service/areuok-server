# Build stage
FROM rust:1.92-bookworm AS builder

# Install PostgreSQL client for build-time verification
RUN apt-get update && apt-get install -y postgresql-client && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock* ./
COPY crates ./crates
COPY .sqlx ./.sqlx

# Build project
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server /app/server

EXPOSE 3000

ENV RUST_LOG=info,server=debug,api=debug,db=debug

CMD ["/app/server"]
