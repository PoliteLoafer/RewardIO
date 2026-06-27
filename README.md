# Axum + React TS Project

![CI](https://github.com/<OWNER>/<REPOSITORY>/actions/workflows/ci.yml/badge.svg)

This project consists of a Rust backend (Axum) and a React TypeScript frontend (Vite), orchestrated with Docker Compose.

## Structure

- `/crates/core`: Pure business logic, models, and traits (domain).
- `/crates/infra`: Infrastructure implementations (database, external services).
- `/crates/api`: Axum web server (entry point).
- `/frontend`: React + TypeScript + Vite app.
- `docker-compose.yml`: Orchestration for both services.

## How to Run

Ensure you have Docker and Docker Compose installed.

1.  **Start the project:**

    ```bash
    docker compose up
    ```

    If you need to rebuild images (for example, after Dockerfile/dependency changes):

    ```bash
    docker compose up --build
    ```

2.  **Access the applications:**

    - **Frontend:** [http://localhost:5173](http://localhost:5173)
    - **Backend API:** [http://localhost:3000/api/hello](http://localhost:3000/api/hello)

## Running Tests

You can run all backend tests using Cargo:

```bash
cargo test
```

Or using the provided script:

```bash
./scripts/test.sh
```

To run tests for a specific crate:

```bash
cargo test -p rewardio-core
cargo test -p rewardio-infra
cargo test -p rewardio-api
```

### Running Tests in Docker

If you want to run tests within the Docker environment (to ensure consistency with production), you can use the following command:

```bash
docker compose run --rm backend cargo test
```

The frontend is configured to proxy `/api` requests to the backend service.

## AI Collaboration Docs

To improve consistency, quality, and token efficiency when working with AI agents, use:

- `PROJECT_STRUCTURE.md` — concise project map and navigation guide.
- `AI_WORKFLOW_RULES.md` — implementation/validation guardrails and AI best practices.

## Database Migrations (SQLx)

Migrations for the backend API are stored in `crates/api/migrations` and use SQLx reversible files:

- `*.up.sql` — apply migration
- `*.down.sql` — rollback migration

Current migrations:

- `20231027120000_create_users_and_roles`
- `20260515141300_create_hello_messages`

### Prerequisites

1. Ensure Postgres is running (for local setup, via Docker Compose):

```bash
docker compose up -d postgres
```

2. Install SQLx CLI (once):

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

### Apply migrations

```bash
sqlx migrate run --source crates/api/migrations --database-url postgres://rewardio:rewardio@localhost:5432/rewardio
```

### Rollback the latest migration

```bash
sqlx migrate revert --source crates/api/migrations --database-url postgres://rewardio:rewardio@localhost:5432/rewardio
```

### Check migration status

```bash
sqlx migrate info --source crates/api/migrations --database-url postgres://rewardio:rewardio@localhost:5432/rewardio
```
