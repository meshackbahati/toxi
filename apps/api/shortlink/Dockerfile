FROM rust:slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY migrations ./migrations
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/shortlink /app/shortlink
COPY migrations ./migrations
ENV SHORTLINK_DB=sqlite:shortlink.db
EXPOSE 3001
CMD ["/app/shortlink"]
