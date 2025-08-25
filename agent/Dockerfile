FROM rust:1.79-bullseye AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

COPY --from=builder /app/target/release/agent .

RUN chown appuser:appuser ./agent

USER appuser

ENTRYPOINT ["./agent"]