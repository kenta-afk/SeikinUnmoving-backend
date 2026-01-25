# Development Dockerfile
FROM rust:1.90-bookworm

# Install mold linker, cmake, clang and build dependencies
RUN apt-get update && \
    apt-get install -y \
    mold \
    cmake \
    pkg-config \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Configure cargo to use mold
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Install sqlx-cli for migrations
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Install cargo-watch for hot reload
RUN cargo install cargo-watch

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY presentation ./presentation
COPY services ./services
COPY .sqlx ./.sqlx

# Copy migration files
COPY db/migrations /app/db/migrations

# Copy the env file
COPY presentation/dev/.env /app/.env

# Build the application in debug mode for faster compilation
ENV SQLX_OFFLINE=true
RUN cargo build --package apiroute

# Copy entrypoint script
COPY presentation/dev/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
