# Toxi Framework Documentation

Documentation for **Toxi**, a Rust web framework for APIs, microservices, serverless functions, and full-stack apps.

## Introduction

Toxi is built on `hyper` and `tokio`. It provides routing, extractors, middleware, and optional crates for database access, authentication, templates, real-time communication, background jobs, and more.

Key features:
- Async request handling with `tokio`
- Type-safe extractors (`Json`, `Path`, `Query`, `State`, `Form`, `Cookies`)
- Middleware system compatible with `tower`
- Optional ORM with auto-diff migrations
- Built-in authentication (JWT, OAuth2, RBAC)
- Server-side template rendering
- WebSocket and SSE support
- CLI for project scaffolding and migrations

## Getting Started

### Installation

Add Toxi to your `Cargo.toml`:

```toml
[dependencies]
toxi = "3.0"
tokio = { version = "1", features = ["full"] }
```

### Basic Example

```rust
use toxi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new();
    app.router_mut().get("/", |_| async {
        Ok(Response::new("Hello, Toxi!"))
    });
    app.run().await
}
```

## Guides

### Building a REST API

```rust
use toxi::prelude::*;

async fn get_users(_req: Request) -> Result<Response> {
    let users = vec!["Alice", "Bob"];
    Ok(json_response!(users))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new();
    app.router_mut().get("/api/users", get_users);
    app.run().await
}
```

### Full-Stack Web Apps

Use `TemplateContext` for server-side rendering:

```rust
use toxi::prelude::*;

async fn profile(_req: Request) -> Result<Response> {
    let ctx = TemplateContext::new("templates");
    let html = ctx.render("profile.html", &ctx)?;
    Ok(Response::html(html))
}
```

### Automatic API Documentation

```toml
[dependencies]
toxi-openapi = "3.0"
```

```rust
use toxi_openapi::{OpenApiBuilder, Info};

async fn api_spec(_req: Request) -> Result<Response> {
    let spec = OpenApiBuilder::new("My API", "3.0.0")
        .description("My API description")
        .build();
    Ok(json_response!(spec))
}
```

### How-To Guides

Step-by-step guides for common tasks:

- **[Building a Fullstack Application](guides/fullstack.md)** - Create a complete web app with Toxi
- **[CLI Tool Usage](guides/cli.md)** - Master the Toxi command-line interface
- **[Static File Serving](guides/static-files.md)** - Serve static assets efficiently
- **[Authorization & Access Control](guides/authorization.md)** - Implement RBAC/PBAC
- **[API Key Authentication](guides/api-keys.md)** - Secure your API with key-based auth

### Project Status

- **[Implementation Status](../STATUS.md)** - Current feature completeness and progress

##  Demo Application

The `examples/demo-app` directory contains a complete full-stack application showcasing:
- **Database Integration**: SQLite with migrations.
- **Authentication**: JWT-based auth.
- **Templates**: Beautiful UI with server-side rendering.
- **API Docs**: Auto-generated OpenAPI documentation.

### Running the Demo
```bash
cd examples/demo-app
cargo run
```
Visit `http://localhost:8080` to see it in action!

##  Advanced Usage

### Middleware
```rust
use toxi::middleware::{CorsLayer, LoggerLayer};

let app = Toxi::new()
    .layer(LoggerLayer::new())
    .layer(CorsLayer::permissive());
```

### Error Handling
Handlers return `Result<Response, Error>`. Errors propagate through middleware and are converted to appropriate HTTP responses.

##  Contributing
We welcome contributions! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

---
Built by the Toxi Team
