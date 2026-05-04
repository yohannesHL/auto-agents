# syntax=docker/dockerfile:1.7
# Rust stub service — replace with a real Rust agent implementation.

FROM rust:1.82-bookworm AS build
WORKDIR /app
COPY Cargo.toml ./
# Pre-fetch dependencies with an empty src for better layer caching
RUN mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo build --release && \
    rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/rust-service /usr/local/bin/rust-service
ENV RUST_SERVICE_PORT=8081
EXPOSE 8081
ENTRYPOINT ["rust-service"]
