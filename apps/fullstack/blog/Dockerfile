FROM rust:slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY templates ./templates
COPY migrations ./migrations
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/target/release/blog /app/blog
COPY templates ./templates
COPY migrations ./migrations
ENV BLOG_DB=sqlite:blog.db
EXPOSE 3003
CMD ["/app/blog"]
