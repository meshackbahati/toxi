# Toxi Web Framework

<div align="center">

<img src="../docs/logo/toxi.svg" width="200" alt="Toxi Logo">

A web framework for Rust. Build APIs, microservices, serverless functions, and full-stack apps.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache-2.0-blue.svg)](../LICENSE)
[![GitHub](https://img.shields.io/badge/github-meshackbahati%2Ftoxi-black)](https://github.com/meshackbahati/toxi)
[![Status](https://img.shields.io/badge/status-beta-yellow.svg)](../STATUS.md)

Built by [Meshack Bahati Ouma](https://github.com/Kyle6012)

</div>

---

## What is Toxi?

Toxi is a Rust web framework for building APIs, microservices, serverless functions, and full-stack applications. It provides routing, extractors, middleware, an ORM with auto-diff migrations, authentication (JWT, OAuth2, RBAC, PBAC), server-side templates, WebSockets and SSE, background job queues, caching, file storage, email delivery, and OpenAPI documentation. The `toxi-cli` tool scaffolds projects, runs migrations, and provides a development server.

Each component is distributed as a separate crate. Use them independently or enable them all through the `toxi` facade with feature flags.

## Crates
- **`toxi-core`**: Routing, hyper HTTP server, and type-safe extractors (`Json`, `Path`, `Query`, `State`, `Form`, `Cookies`, `Body`).
- **`toxi-db`**: ORM with async validation rules (`length`, `range`, `email`, `url`, `regex`, `custom`, `unique`), relationships (`has_many`, `has_one`, `belongs_to`), and auto-diff schema migrations.
- **`toxi-auth`**: RBAC/PBAC, API keys, JWT sessions, and OAuth2 integration.
- **`toxi-realtime`**: WebSockets, Server-Sent Events (SSE), and Redis-backed event broadcasting.
- **`toxi-queue`**: Background job execution with automatic retries and dead-letter queues.
- **`toxi-cache`**: Caching with in-memory and Redis backends.
- **`toxi-storage`**: Storage API with local disk and S3/Cloudinary/ImageKit backends.
- **`toxi-template`**: Lightweight server-side rendering (SSR) templates.
- **`toxi-openapi`**: Automatic Swagger UI/OpenAPI 3.0 document generation.
- **`toxi-cli`**: Project scaffolding, migrations, code generators, and an interactive console.

### ORM Features

The built-in ORM provides:
- **Compile-time checks**: Procedural macros verify handler extractor bindings and model queries at compile time.
- **Eager loading**: Static helper methods (`eager_load_posts`, `eager_load_profile`) execute batched SQL `IN` queries to prevent N+1 queries.
- **Auto-diff migrations**: The CLI parses your Rust struct models, diffs them against the live database schema, and generates migrations automatically.

### Benchmarks

Public benchmarks are not yet available. See [STATUS.md](../STATUS.md) for current feature completeness.

##  Installation

Install the Toxi CLI tool to get started:

```bash
# Install from source (recommended for development)
cargo install --path ../toxi-cli

# Or install from crates.io (when published)
cargo install toxi-cli
```

##  Getting Started

### Quick Start

Create a new Toxi project in seconds:

```bash
toxi new my-app
```

Then run your application:

```bash
# Navigate to your project
cd my-app

# Start the development server
toxi dev
```

Your application will be available at `http://127.0.0.1:8080`.

### Hello World Example

Here's a simple "Hello World" example:

```rust
use toxi::prelude::*;

async fn hello(_req: ToxiRequest) -> Result<ToxiResponse> {
    Ok(response::text("Hello, Toxi!"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().unwrap();
    let mut app = Application::new(config);
    app.router_mut().get("/", hello);
    app.run().await
}
```

> **Alternative**: The manual approach using `Router::new()` → `router.get(...)` → `Server::new(router).listen(addr)` works identically.

### Using Extractors

Extractors deserialize request data from common formats:

```rust
use toxi::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Handle JSON requests
async fn create_user(Json(payload): Json<CreateUser>) -> Result<ToxiResponse> {
    // payload contains deserialized JSON data
    Ok(json_response!({
        "message": "User created successfully",
        "user": payload
    }))
}

// Handle form data
async fn create_user_form(Form(payload): Form<CreateUser>) -> Result<ToxiResponse> {
    // payload contains deserialized form data
    Ok(json_response!({
        "message": "User created from form",
        "user": payload
    }))
}

// Handle query parameters
async fn search_users(Query(params): Query<CreateUser>) -> Result<ToxiResponse> {
    // params contains deserialized query parameters
    Ok(json_response!({
        "message": "Search results",
        "query": params
    }))
}

// Handle path parameters
#[derive(Deserialize)]
struct UserId {
    id: u32,
}

async fn get_user(Path(params): Path<UserId>) -> Result<ToxiResponse> {
    // params.id contains the path parameter
    Ok(json_response!({
        "id": params.id,
        "name": "Sample User"
    }))
}

// Access cookies
async fn get_cookies(Cookies(cookies): Cookies) -> Result<ToxiResponse> {
    // cookies is a HashMap<String, String> containing request cookies
    Ok(json_response!(cookies))
}

// Access raw body
async fn handle_raw_body(Body(body): Body) -> Result<ToxiResponse> {
    // body is a String containing the raw request body
    Ok(response::text(format!("Received {} characters", body.len())))
}
```

##  Core Concepts

### Routers and Handlers

Toxi uses a clean routing system with async handlers:

```rust
use toxi::prelude::*;

async fn home(_req: ToxiRequest) -> Result<ToxiResponse> {
    Ok(response::html("<h1>Welcome to Toxi!</h1>"))
}

async fn api_handler(_req: ToxiRequest) -> Result<ToxiResponse> {
    Ok(json_response!({
        "message": "Hello from API"
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().unwrap();
    let mut app = Application::new(config);
    
    // Basic routes
    app.router_mut().get("/", home);
    app.router_mut().get("/api", api_handler);
    
    // Routes with parameters
    app.router_mut().get("/users/:id", get_user);
    app.router_mut().post("/users", create_user);
    app.router_mut().put("/users/:id", update_user);
    app.router_mut().delete("/users/:id", delete_user);
    
    app.run().await
}
```

> **Alternative**: The manual approach using `Router::new()` → `router.get(...)` → `Server::new(router).listen(addr)` works identically.

### Request Handling

Toxi provides several extractors to handle different types of requests:

- **Json<T>**: Extracts and deserializes JSON from request body
- **Form<T>**: Extracts and deserializes form data
- **Query<T>**: Extracts and deserializes query parameters
- **Path<T>**: Extracts and deserializes path parameters
- **Cookies**: Extracts cookies as HashMap
- **Body**: Extracts raw request body as String
- **State<T>**: Extracts application state from request extensions

### API Versioning

Toxi supports multiple API versioning strategies:

```rust
use toxi::prelude::*;

// URL-based versioning
async fn api_v1_handler(_req: ToxiRequest) -> Result<ToxiResponse> {
    Ok(json_response!({
        "version": "1.0",
        "data": "API v1 response"
    }))
}

async fn api_v2_handler(_req: ToxiRequest) -> Result<ToxiResponse> {
    Ok(json_response!({
        "version": "2.0",
        "data": "API v2 response",
        "enhanced": true
    }))
}

// Version-specific routes
router.get("/api/v1/users", api_v1_handler);
router.get("/api/v2/users", api_v2_handler);
```

### Error Handling

Toxi provides comprehensive error handling with appropriate HTTP status codes:

```rust
use toxi::prelude::*;

async fn protected_route(_req: ToxiRequest) -> Result<ToxiResponse> {
    // Simulate a forbidden request
    Err(ToxiError::Forbidden("Access denied".to_string()))
}

async fn conflict_route(_req: ToxiRequest) -> Result<ToxiResponse> {
    // Simulate a conflict error
    Err(ToxiError::Conflict("Resource conflict".to_string()))
}

async fn validation_route(Json(data): Json<MyData>) -> Result<ToxiResponse> {
    // Validate the data
    if data.is_valid() {
        Ok(json_response!("Valid"))
    } else {
        Err(ToxiError::Validation("Invalid data".to_string()))
    }
}
```

##  Documentation

Complete documentation is available in the [docs/](../docs/) directory:

- [Getting Started](../docs/getting-started.md) - Your first Toxi application
- [Core Concepts](../docs/core-concepts.md) - Fundamental architecture and concepts
- [API Documentation](../docs/api.md) - Complete API reference
- [Framework Features](../docs/framework.md) - Framework features and capabilities
- [Added Features](../docs/features-added.md) - Recently added features and improvements
- [Database Guide](../docs/database.md) - ORM and database operations
- [Authentication Guide](../docs/authentication.md) - Authentication and authorization
- [Templating Guide](../docs/templating.md) - Server-side rendering
- [Middleware Guide](../docs/middleware.md) - Adding functionality with middleware
- [CLI Tools](../docs/cli.md) - Command-line interface

##  Architecture

Toxi is composed of modular crates that can be used independently:

| Crate | Description |
|-------|-------------|
| `toxi` | Main crate with prelude and convenience exports |
| `toxi-core` | Core HTTP server, routing, and extractors |
| `toxi-db` | Database ORM with migrations and relationships |
| `toxi-auth` | Authentication and authorization |
| `toxi-template` | Template engine for server-side rendering |
| `toxi-middleware` | Common middleware implementations |
| `toxi-cli` | Command-line tools for project management |
| `toxi-config` | Configuration management |
| `toxi-cache` | Caching utilities |
| `toxi-queue` | Background job processing |
| `toxi-realtime` | Real-time features (WebSockets, SSE) |
| `toxi-mail` | Email sending capabilities |
| `toxi-storage` | File storage (local and S3) |
| `toxi-openapi` | OpenAPI/Swagger integration |
| `toxi-macros` | Procedural macros |
| `toxi-security` | Security utilities |
| `toxi-testing` | Testing utilities |
| `toxi-utils` | Common utilities |

##  Testing

Toxi provides comprehensive testing utilities:

```rust
use toxi::prelude::*;
use toxi_testing::TestClient;

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_hello_endpoint() {
        let mut router = Router::new();
        router.get("/", hello);
        
        let client = TestClient::new(router);
        let response = client.get("/").send().await;
        
        assert_eq!(response.status(), 200);
        assert_eq!(response.text().await, "Hello, Toxi!");
    }
}
```

##  Deployment

Deploy your Toxi application with any Rust-compatible hosting provider:

```bash
# Build for release
cargo build --release

# Run the application
./target/release/my-app
```

For containerized deployment:

```dockerfile
FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/my-app"]
```

##  Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md).

##  License

MIT OR Apache-2.0 - see [LICENSE](LICENSE) for details.
