FROM rust:slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/echo /app/echo
EXPOSE 3002
CMD ["/app/echo"]
