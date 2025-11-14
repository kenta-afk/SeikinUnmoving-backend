# Build stage
FROM rust:1.90-bookworm AS builder

# Install mold linker and build dependencies
RUN apt-get update && \
    apt-get install -y clang mold && \
    rm -rf /var/lib/apt/lists/*

# Configure cargo to use mold
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features sqlite

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY apiroute ./apiroute
COPY services ./services

# Build the application
RUN cargo build --release --package apiroute

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /build/target/release/apiroute /app/apiroute

# Copy sqlx-cli for migrations
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Copy migration files
COPY db/migrations /app/db/migrations
COPY .sqlx-config.toml /app/.sqlx-config.toml

# Copy the env file
COPY apiroute/dev/.env /app/.env

# Copy entrypoint script
COPY apiroute/dev/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
