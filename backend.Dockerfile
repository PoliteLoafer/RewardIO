# Build stage
FROM rust:1.83-slim as builder

WORKDIR /app
COPY . .

# Build the specific api crate
RUN cargo build --release -p rewardio-api

# Final stage
FROM debian:bookworm-slim
WORKDIR /app

# Copy the binary from the workspace target directory
COPY --from=builder /app/target/release/rewardio-api /app/backend

EXPOSE 3000
CMD ["./backend"]
