# Project Structure

This file is a fast navigation map for humans and AI agents.
Use it as the first context source before opening many files.

## High-level layout

- `Cargo.toml` / `Cargo.lock` — Rust workspace configuration.
- `crates/` — backend Rust workspace crates.
  - `crates/core` — domain layer: entities, service traits, business logic.
  - `crates/infra` — infrastructure adapters: repositories, DB-backed implementations.
  - `crates/api` — HTTP API (Axum), app bootstrap, config, logging, migrations.
- `crates/api/migrations` — SQLx reversible migrations (`*.up.sql`, `*.down.sql`).
- `frontend/` — React + TypeScript + Vite client.
- `docker-compose.yml` — local orchestration (`postgres`, `backend`, `frontend`).
- `backend.Dockerfile` — multi-stage backend image build.
- `scripts/` — utility scripts and container entrypoints.
- `README.md` — developer quick start and migration commands.

## Backend architecture (Rust)

- API handlers (`crates/api/src/**`) depend on service traits from `core`.
- `core` depends on abstractions (`UserRepository`, services), not concrete DB/file code.
- `infra` provides concrete implementations (for example `PostgresUserRepository`).
- Runtime wiring happens in `crates/api/src/main.rs`:
  - load config
  - initialize logger
  - connect Postgres + run migrations
  - build repositories/services
  - start Axum app

## Data flow examples

- Auth flow:
  - endpoint: `crates/api/src/auth/handlers/mod.rs`
  - service: `crates/core/src/services/auth.rs`
  - repository impl: `crates/infra/src/user_repository.rs`
- Hello message flow:
  - endpoint: `crates/api/src/hello/**`
  - service/repository wiring: `crates/api/src/main.rs`

## Configuration and runtime

- Main config parsing/validation: `crates/api/src/config.rs`
- Environment file used by Docker Compose: `.env`
- Postgres URL is provided via `REWARDIO__POSTGRES_URL` in `docker-compose.yml`

## Testing map

- Full backend tests: `cargo test -p rewardio-api`
- Workspace tests: `cargo test`
- App-level route tests are in `crates/api/src/app.rs` (`#[cfg(test)]` module)
- Rule for new features: add meaningful tests that verify real scenarios and observable outcomes (API status/body, state changes, side effects), not coverage-only execution.

## Migration workflow

- Apply: `sqlx migrate run --source crates/api/migrations --database-url <url>`
- Revert latest: `sqlx migrate revert --source crates/api/migrations --database-url <url>`
- Status: `sqlx migrate info --source crates/api/migrations --database-url <url>`
