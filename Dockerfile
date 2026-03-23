FROM lukemathwalker/cargo-chef:latest-rust-1.93.0 AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef:latest-rust-1.93.0 AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin loomox

FROM debian:trixie-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --create-home appuser
USER appuser
WORKDIR /usr/local/bin
COPY --from=builder /app/target/release/loomox /usr/local/bin/loomox
COPY config.docker.env /usr/local/bin/.env
ENV RUST_LOG=info
EXPOSE 3000
ENTRYPOINT ["loomox"]
