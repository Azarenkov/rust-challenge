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
        "address": "0xPSxka53Qdp",
        "total_volume": 8686.785780697088,
        "avg_buy_price": 1.0487974699811131,
        "avg_sell_price": 1.0303341591954556,
        "max_balance": 429.87521997328713
      },
      ...
    ]
    ```

  - **Error Responses:**
    - `405 Method Not Allowed` for unsupported methods (POST, PUT, DELETE).

## Server Configuration
```bash
    PORT=<your_port>
    CLICKHOUSE_URL=<your_clickhouse_url>
    CLICKHOUSE_USER=<your_clickhouse_user>
    CLICKHOUSE_PASSWORD=<your_clickhouse_password>
    DATA_GENERATION_COUNT=40
    RUST_LOG=info --Optional
```
