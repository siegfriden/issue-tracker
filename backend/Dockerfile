FROM rust:1.85-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies by building them first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/generate_openapi.rs && \
    echo "fn main() {}" > src/bin/migrate_up.rs && \
    echo "fn main() {}" > src/bin/migrate_down.rs && \
    cargo build --release && \
    rm -rf src

# Build the actual application
COPY src/ src/
COPY migrations/ migrations/

RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/issue-tracker-api /usr/local/bin/
COPY --from=builder /app/target/release/migrate-up /usr/local/bin/
COPY --from=builder /app/target/release/migrate-down /usr/local/bin/
COPY migrations/ /app/migrations/

WORKDIR /app

EXPOSE 8080

CMD ["issue-tracker-api"]
