FROM rust:slim AS build
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
COPY --from=build /app/target/release/pastebin /app/pastebin
COPY templates ./templates
COPY public ./public
COPY migrations ./migrations
ENV PASTEBIN_DB=sqlite:pastebin.db
EXPOSE 3006
CMD ["/app/pastebin"]
