# Build tooling stage
FROM rust:1.94-slim as chef

WORKDIR /app
RUN cargo install cargo-chef

# Generate dependency recipe (changes only when Cargo manifests/lockfiles change)
FROM chef as planner

WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies and application using cache-friendly layers
FROM chef as builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release -p rewardio-api

# Final stage
FROM debian:bookworm-slim
WORKDIR /app

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates gosu && rm -rf /var/lib/apt/lists/*

# Create logs directories and ensure they are writable by appuser
RUN mkdir -p logs/back_logs logs/front_logs && \
    chown -R appuser:appuser logs && \
    chmod -R 755 logs

# Copy the binary from the workspace target directory
COPY --from=builder /app/target/release/rewardio-api /app/backend
COPY --from=builder /app/crates/api/migrations /app/migrations
RUN chown appuser:appuser /app/backend
RUN chown -R appuser:appuser /app/migrations

COPY scripts/docker/backend-entrypoint.sh /usr/local/bin/backend-entrypoint.sh
RUN chmod +x /usr/local/bin/backend-entrypoint.sh

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/backend-entrypoint.sh"]
CMD ["./backend"]
