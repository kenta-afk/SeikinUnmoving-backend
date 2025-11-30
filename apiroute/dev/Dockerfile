# Development Dockerfile
FROM rust:1.90-bookworm

# Install mold linker and build dependencies
RUN apt-get update && \
    apt-get install -y clang mold && \
    rm -rf /var/lib/apt/lists/*

# Configure cargo to use mold
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Install cargo-watch for hot reload
RUN cargo install cargo-watch

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY apiroute ./apiroute
COPY services ./services
COPY .sqlx ./.sqlx

# Copy migration files
COPY db/migrations /app/db/migrations

# Copy the env file
COPY apiroute/dev/.env /app/.env

# Build the application in debug mode for faster compilation
# Use offline mode with pre-generated query metadata
ENV SQLX_OFFLINE=true
RUN cargo build --package apiroute

# Copy entrypoint script
COPY apiroute/dev/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
