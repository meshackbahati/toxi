FROM rust:1.89-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY templates ./templates
COPY public ./public
COPY migrations ./migrations
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/taskboard /app/taskboard
COPY templates ./templates
COPY public ./public
COPY migrations ./migrations
ENV TASKBOARD_DB=sqlite:taskboard.db
EXPOSE 3000
CMD ["/app/taskboard"]
