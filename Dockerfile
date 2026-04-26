FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -s /bin/bash appuser

WORKDIR /app

COPY target/release/rust-crud-api /app/

RUN chown -R appuser:appuser /app

USER appuser

EXPOSE 8080

ENV RUST_LOG=debug
ENV APP_SERVER__PORT=8080

ENTRYPOINT ["/app/rust-crud-api"]