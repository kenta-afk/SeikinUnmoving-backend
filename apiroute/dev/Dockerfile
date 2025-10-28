# Build stage
FROM rust:1.90-bookworm AS builder

# Install mold linker
RUN apt-get update && \
    apt-get install -y clang mold && \
    rm -rf /var/lib/apt/lists/*

# Configure cargo to use mold
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

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

# Copy the env file
COPY apiroute/dev/.env /app/.env

EXPOSE 8080

CMD ["/app/apiroute"]
