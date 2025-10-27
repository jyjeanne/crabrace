# Multi-stage Dockerfile for Crabrace
# Stage 1: Build the application
FROM rust:1.75-slim as builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /usr/src/crabrace

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release with optimizations
RUN cargo build --release --locked

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false -u 1000 crabrace

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /usr/src/crabrace/target/release/crabrace /app/crabrace

# Change ownership to non-root user
RUN chown -R crabrace:crabrace /app

# Switch to non-root user
USER crabrace

# Expose the application port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV PORT=8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/crabrace", "--version"] || exit 1

# Run the application
CMD ["/app/crabrace"]
