FROM rust:1.90-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

RUN cargo build --release -F telegram

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/cukiller-backend /usr/local/bin/cukiller-backend
COPY --from=builder /app/migrations ./migrations

EXPOSE 3000

CMD ["/usr/local/bin/cukiller-backend"]
