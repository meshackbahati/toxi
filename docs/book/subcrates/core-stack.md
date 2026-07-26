# Core Stack Crates

## `toxi`

Use when you want a single entry point for framework features.

Main exports:

- `toxi_core::*`
- feature-gated re-exports (`db`, `auth`, `queue`, `cache`, `realtime`, `template`, `mail`, `storage`, `security`, `utils`)
- `prelude` module

## `toxi-core`

Primary APIs:

- modules: `error`, `extract`, `request`, `response`, `router`, `server`, `tls`, `types`, `versioning`, `cookie`
- key re-exports:
- `Router`, `Server`
- `Request`, `Response`
- extractors: `Json`, `Path`, `Query`, `State`, `Form`, `Cookies`, `Body`

Typical scenario:

1. create `Router`
2. register routes
3. start `Server`

## `toxi-middleware`

Main APIs:

- `LoggerLayer`
- `RequestIdLayer`
- `SecurityHeadersLayer`
- `CsrfLayer`
- `RateLimiter`
- `TimeoutMiddleware`
- `CacheLayer`

Use to compose middleware with `tower::ServiceBuilder`.

## `toxi-config`

Typed config structs:

- `Config`
- `AppConfig`
- `ServerConfig`
- `DatabaseConfig`
- `CacheConfig`
- `QueueConfig`
- `SecurityConfig`

Use for environment-aware startup configuration.

## `toxi-utils`

Main utility groups:

- date helpers
- id generation (`generate_id`, `generate_uuid`, `generate_short_id`, `generate_numeric_id`)
- string helpers (`slugify`, `truncate`, `capitalize`, `random_string`, `camel_case`, `snake_case`)
- input validation helpers
