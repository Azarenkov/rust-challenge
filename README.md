# Rust Challenge

**Rust Challenge** is a project designed to demonstrate working with Rust, Actix Web, and ClickHouse. It generates and stores transfer data, then provides API endpoints for retrieving user statistics.

## Makefile Commands

- `make run`
  Start the application with the database.
- `make test-unit`
  Run unit tests.
- `make test-quick`
  Run a quick test without the database.
- `make test-integration`
  Run an integration test (requires the database).
- `make setup-db`
  Start ClickHouse via Docker.
- `make cleanup`
  Stop Docker containers.
- `make status`
  Show status of Docker containers and ClickHouse health.

## API Documentation

- **GET `/api/v1/stats/get_all`**
  Returns an array of statistics in JSON format.

  - **Response:**
    `200 OK` – Array of user stats objects

    Example:
    ```json
    [
      {
        "user_id": "string",
        "total_volume": 1234.56,
        "max_balance": 789.01
      },
      ...
    ]
    ```

  - **Error Responses:**
    - `405 Method Not Allowed` for unsupported methods (POST, PUT, DELETE).

## Server Configuration
```bash
    PORT=8080
    CLICKHOUSE_URL=http://localhost:8123
    CLICKHOUSE_USER=default
    CLICKHOUSE_PASSWORD=
    DATA_GENERATION_COUNT=100
    RUST_LOG=info --Optional
```
