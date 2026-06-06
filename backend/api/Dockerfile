FROM rust:1.95-slim AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install --yes --no-install-recommends musl-tools \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add x86_64-unknown-linux-musl

COPY api/src/Cargo.toml Cargo.toml
COPY api/src/Cargo.lock Cargo.lock
COPY api/Cargo.toml api/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml

RUN mkdir -p api/src migration/src \
    && echo "fn main() {}" > api/src/main.rs \
    && touch api/src/lib.rs migration/src/lib.rs \
    && echo "fn main() {}" > migration/src/main.rs \
    && cargo build --locked --release --target x86_64-unknown-linux-musl -p keepalive-api \
    && rm -rf api/src migration/src

COPY api/src api/src
COPY migration/src migration/src
RUN touch api/src/main.rs api/src/lib.rs migration/src/lib.rs migration/src/main.rs \
    && cargo build --locked --release --target x86_64-unknown-linux-musl -p keepalive-api

FROM gcr.io/distroless/static-debian12:nonroot
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/keepalive-api /app/keepalive-api
EXPOSE 3000
CMD ["/app/keepalive-api"]
