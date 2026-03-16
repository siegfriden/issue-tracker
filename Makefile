db-create:
	createdb issue_tracker
db-reset:
	dropdb --if-exists issue_tracker
	createdb issue_tracker
db-up:
	cd backend && cargo run --bin migrate-up
db-down:
	cd backend && cargo run --bin migrate-down

run-be:
	cd backend && cargo run --bin issue-tracker-api
run-fe:
	cd frontend && npm run dev
run-all:
	@echo "Starting backend and frontend... (Press Ctrl+C to stop)"
	@trap 'kill 0' SIGINT; make run-be & make run-fe & wait

lint-be:
	cd backend && cargo clippy -- -D warnings
lint-fe:
	cd frontend && npm run lint
lint-all:
	@echo "Linting backend and frontend..."
	@make lint-be
	@make lint-fe

fmt-be:
	cd backend && cargo fmt
fmt-fe:
	cd frontend && npm run format
fmt-all:
	@echo "Formatting backend and frontend..."
	@make fmt-be
	@make fmt-fe

openapi:
	cd backend && cargo run --bin generate_openapi -- ../references/openapi.yaml
