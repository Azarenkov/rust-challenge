.PHONY: help test-unit test-integration test-quick setup-db cleanup run

help:
	@echo "Rust Challenge"
	@echo ""
	@echo "make run              - Запуск приложения с БД"
	@echo "make test-unit        - Unit тесты"
	@echo "make test-quick       - Быстрый тест без БД"
	@echo "make test-integration - Интеграционный тест"
	@echo "make setup-db         - Запуск ClickHouse"
	@echo "make cleanup          - Остановка контейнеров"

run: setup-db
	@echo "Starting application..."
	@trap 'echo "\nStopping containers..."; docker-compose down; exit 0' INT TERM; \
	cargo run; \
	echo "Stopping containers..."; \
	docker-compose down

test-unit:
	@echo "Running unit tests..."
	cargo test --lib

test-quick:
	@echo "Running quick test..."
	cargo test --test integration_test test_config -- --nocapture

setup-db:
	@echo "Starting ClickHouse..."
	docker-compose up -d clickhouse
	@echo "Waiting for ClickHouse..."
	sleep 8
	@curl -f http://localhost:8123/ping > /dev/null 2>&1 && \
		echo "ClickHouse ready" || \
		echo "❌ ClickHouse failed"

test-integration: setup-db
	@echo "Running integration test..."
	@trap 'echo "\nStopping containers..."; docker-compose down; exit 0' INT TERM; \
	cargo test --test integration_test test_integration_with_run -- --ignored --nocapture; \
	echo "Stopping containers..."; \
	docker-compose down

cleanup:
	@echo "Cleaning up..."
	docker-compose down

status:
	@echo "Docker containers status:"
	@docker ps | grep -E "(clickhouse|CONTAINER)" || echo "No containers running"
	@echo "ClickHouse health:"
	@curl -f http://localhost:8123/ping > /dev/null 2>&1 && \
		echo "ClickHouse is running" || \
		echo "❌ ClickHouse is not available"
