# Build stage
FROM rust:1.94-slim as builder

WORKDIR /app
COPY . .

# Build the specific api crate
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
RUN chown appuser:appuser /app/backend

COPY scripts/docker/backend-entrypoint.sh /usr/local/bin/backend-entrypoint.sh
RUN chmod +x /usr/local/bin/backend-entrypoint.sh

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/backend-entrypoint.sh"]
CMD ["./backend"]
