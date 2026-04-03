db-create:
	createdb issue_tracker
db-reset:
	dropdb --if-exists issue_tracker
	createdb issue_tracker
db-up:
	cd backend && cargo run --bin migrate-up
db-down:
	cd backend && cargo run --bin migrate-down

run-api:
	cd backend && cargo run --bin issue-tracker-api
run-ui:
	cd frontend && npm run dev
run-all:
	@echo "Starting backend and frontend... (Press Ctrl+C to stop)"
	@trap 'kill 0' SIGINT; make run-api & make run-ui & wait

lint-api:
	cd backend && cargo clippy -- -D warnings
lint-ui:
	cd frontend && npm run lint
lint-all:
	@echo "Linting backend and frontend..."
	@make lint-api
	@make lint-ui

fmt-api:
	cd backend && cargo fmt
fmt-ui:
	cd frontend && npm run format
fmt-all:
	@echo "Formatting backend and frontend..."
	@make fmt-api
	@make fmt-ui

openapi:
	cd backend && cargo run --bin generate_openapi -- ../references/openapi.yaml
