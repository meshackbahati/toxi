# Toxi

<div align="center">

<img src="docs/logo/toxi.svg" width="200" alt="Toxi Logo">

A web framework for Rust (previously Oxidite). Build APIs, microservices, serverless functions, and full-stack apps.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache-2.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-3.1.0-blue.svg)](Cargo.toml)
[![GitHub](https://img.shields.io/badge/github-meshackbahati%2Ftoxi-black)](https://github.com/meshackbahati/toxi)

</div>

---

## What is Toxi?

Toxi is a Rust web framework for building APIs, microservices, serverless functions, and full-stack applications. It provides routing, extractors, middleware, an ORM with auto-diff migrations, authentication, server-side templates, real-time communication, background job queues, caching, file storage, email delivery, and OpenAPI documentation. Each component is a separate crate. Use `toxi-core` alone for a minimal API server, or enable the full stack through the `toxi` facade.

## Quick Start

```toml
[dependencies]
toxi = "3.1.0"
tokio = { version = "1", features = ["full"] }
```

```rust
use toxi::prelude::*;

async fn hello(_req: Request) -> Result<Response> {
    Ok(json_response!({ "message": "Hello from Toxi!" }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new(Config::load().unwrap());
    app.router_mut().get("/", hello);
    app.run().await
}
```

## Core Concepts

### Extractors

Handler parameters are automatically extracted from the request:

```rust
use toxi::prelude::*;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(
    State(db): State<Arc<DbPool>>,
    Json(body): Json<CreateUser>,
) -> Result<Response> {
    let user = User::create(&db, &body.name, &body.email).await?;
    Ok(json_response!({ "id": user.id }))
}
```

Built-in extractors: `Json<T>`, `Path<T>`, `Query<T>`, `State<T>`, `Form<T>`, `Cookies`, `Body<T>`, `WebSocketUpgrade`.

### Routing

Regex-based routing with path parameters, wildcards, and middleware:

```rust
let mut router = Router::new();
router.get("/users/:id", get_user);
router.post("/users", create_user);
router.delete("/users/:id", delete_user);
```

### HTTP/2 Support

Toxi supports HTTP/1.1 and HTTP/2 out of the box:

```rust
Server::new(router)
    .with_http_version(HttpVersion::Http2)
    .listen(addr)
    .await
```

When behind a TLS-terminating proxy, use `HttpVersion::Http::Auto` for ALPN negotiation.

### WebSocket Support

Native WebSocket upgrade with authenticated context:

```rust
async fn ws_handler(ws: WebSocketUpgrade) -> Result<Response> {
    Ok(ws.on_upgrade(|socket, extensions| async move {
        // Handle the WebSocket connection
    }))
}
```

## Modular by Design

Each crate in the Toxi ecosystem is independent. Pick only what you need:

| Crate | Purpose |
|-------|---------|
| **`toxi`** | Unified facade — re-exports everything |
| **`toxi-core`** | HTTP kernel, routing, extractors, server |
| **`toxi-macros`** | `#[derive(Model)]` and other proc macros |
| **`toxi-db`** | ORM with relationships, migrations, eager loading |
| **`toxi-auth`** | JWT, OAuth2, RBAC, 2FA, API keys |
| **`toxi-realtime`** | WebSocket, SSE, event broadcasting |
| **`toxi-middleware`** | CORS, logging, compression, rate limiting, CSRF |
| **`toxi-config`** | TOML config with env variable overrides |
| **`toxi-cache`** | In-memory and Redis caching |
| **`toxi-queue`** | Background jobs with Postgres and Redis backends |
| **`toxi-template`** | Server-side rendering engine |
| **`toxi-mail`** | SMTP email delivery |
| **`toxi-storage`** | Local and S3 file storage |
| **`toxi-security`** | Crypto primitives, hashing, sanitization |
| **`toxi-utils`** | String, date, validation helpers |
| **`toxi-openapi`** | OpenAPI 3.0 schema generation |
| **`toxi-graphql`** | GraphQL integration |
| **`toxi-testing`** | Test utilities and mock servers |
| **`toxi-plugin`** | Plugin lifecycle hooks |
| **`toxi-cli`** | Scaffolding, `tinker` REPL, dev server |

### Minimal Setup

```toml
# Just routing — nothing else
[dependencies]
toxi-core = "3.1.0"
```

### With Database

```toml
[dependencies]
toxi-core = "3.1.0"
toxi-db = "3.1.0"
```

### Full Stack

```toml
[dependencies]
toxi = { version = "3.1.0", features = ["full"] }
```

## Feature Flags

| Feature | Enables |
|---------|---------|
| `database` | ORM, migrations, relationships |
| `auth` | JWT, OAuth2, RBAC, 2FA |
| `queue` | Background job processing |
| `cache` | Response and data caching |
| `realtime` | WebSocket and SSE |
| `templates` | Server-side rendering |
| `mail` | Email delivery |
| `storage` | File storage (local + S3) |
| `security` | Crypto and hashing |
| `utils` | String and date helpers |
| `graphql` | GraphQL API |
| `plugin` | Plugin system |
| `full` | Everything above |

## ORM

Toxi includes a custom ORM built on `sqlx` with `#[derive(Model)]`:

```rust
use toxi_db::Model;

#[derive(Model)]
#[model(table = "users")]
struct User {
    id: i64,
    name: String,
    email: String,
}

// Generated methods:
// User::find_by_id(&db, 1).await?
// User::create(&db, "Alice", "alice@example.com").await?
// User::query().filter_eq("name", "Alice").fetch_all(&db).await?
// user.posts().get(&db).await?  (relationship)
```

Features: async validation, soft deletes, eager loading (N+1 prevention), auto-diff migrations, savepoint transactions.

## CLI

Install the CLI for scaffolding and development:

```bash
cargo install --path toxi-cli
```

Commands:
- `toxi new <project>` — scaffold a new project
- `toxi dev` — hot-reload development server
- `toxi migrate` — run database migrations
- `toxi tinker` — interactive REPL
- `toxi generate` — code generation

## Architecture

```text
┌─────────────┐     ┌──────────────┐     ┌──────────────┐
│   Client     │────▶│    Server     │────▶│   Router     │
│  (HTTP/WS)   │     │  (hyper)      │     │  (routes)    │
└─────────────┘     └──────────────┘     └──────┬───────┘
                                                  │
                                          ┌───────▼───────┐
                                          │   Handler     │
                                          │  (extractors) │
                                          └───────────────┘
```

Built on:
- **hyper** — HTTP/1.1 and HTTP/2
- **tokio** — async runtime
- **tower** — middleware composition
- **sqlx** — database access

## Contributing

Contributions welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT OR Apache-2.0 — see [LICENSE](LICENSE).
