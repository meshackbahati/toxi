# Getting Started with Toxi

This guide will help you build your first web application with Toxi.

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Create a New Project

```bash
cargo new my-toxi-app
cd my-toxi-app
```

### Add Toxi

Add Toxi to your `Cargo.toml`:

```toml
[dependencies]
toxi = "3.0"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

## Your First Route

Replace the contents of `src/main.rs`:

```rust
use toxi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Router::new();
    
    app.get("/", hello);
    app.get("/users/:id", get_user);
    
    println!("Server running on http://127.0.0.1:3000");
    
    Server::new(app)
        .listen("127.0.0.1:3000".parse().unwrap())
        .await
}

async fn hello(_req: ToxiRequest) -> Result<ToxiResponse> {
    Ok(Response::text("Hello, Toxi!"))
}

async fn get_user(Path(params): Path<std::collections::HashMap<String, String>>) -> Result<ToxiResponse> {
    let user_id = params.get("id").unwrap();
    Ok(ToxiResponse::text(format!("User ID: {}", user_id)))
}
```

### Run Your App

```bash
cargo run
```

Visit `http://localhost:3000` in your browser!

## JSON API Example

Let's create a simple JSON API:

```rust
use toxi::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Router::new();
    
    app.get("/api/users", list_users);
    app.get("/api/users/:id", get_user);
    app.post("/api/users", create_user);
    
    Server::new(app)
        .listen("127.0.0.1:3000".parse().unwrap())
        .await
}

async fn list_users(_req: ToxiRequest) -> Result<Json<Vec<User>>> {
    let users = vec![
        User { id: 1, name: "Alice".into(), email: "alice@example.com".into() },
        User { id: 2, name: "Bob".into(), email: "bob@example.com".into() },
    ];
    Ok(Json(users))
}

async fn get_user(Path(params): Path<std::collections::HashMap<String, String>>) -> Result<Json<User>> {
    let id = params.get("id").unwrap().parse().unwrap();
    Ok(Json(User {
        id,
        name: "Alice".into(),
        email: "alice@example.com".into(),
    }))
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

async fn create_user(Json(data): Json<CreateUserRequest>) -> Result<Json<User>> {
    Ok(Json(User {
        id: 3,
        name: data.name,
        email: data.email,
    }))
}
```

## Using Middleware

Add CORS and logging:

```rust
use toxi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Router::new();
    app.get("/", hello);
    
    // Add middleware
    let app = ServiceBuilder::new()
        .layer(LoggerLayer)
        .layer(CorsLayer::permissive())
        .service(app);
    
    Server::new(app)
        .listen("127.0.0.1:3000".parse().unwrap())
        .await
}
```

## Next Steps

- [Database Guide](database.md) - Learn about the ORM
- [Authentication Guide](authentication.md) - Add user authentication
- [Background Jobs](background-jobs.md) - Process async tasks
- [Testing Guide](testing.md) - Test your application

## Using the CLI Tool

Install the `toxi-cli` package to get the `toxi` executable:

```bash
cargo install toxi-cli
```

Create and run a project:

```bash
toxi new myapp --type fullstack
cd myapp
toxi migrate
toxi dev
```

Generate code:

```bash
toxi generate model User
toxi generate model Profile display_name:string
toxi generate controller UserController
toxi generate middleware AuthMiddleware
toxi generate route users
toxi generate service Billing
toxi generate validator CreateUser
toxi generate job SendDigest
toxi generate policy Post
toxi generate event UserSignedUp
toxi generate migration create_users_table
toxi generate seeder users_seed
```

## Feature Flags

Use only what you need:

```toml
# Full framework (default)
[dependencies]
toxi = "3.0"

# Minimal (HTTP only)
[dependencies]
toxi = { version = "3.0", default-features = false }

# Custom features
[dependencies]
toxi = { version = "3.0", features = ["database", "auth", "queue"] }
```

Available features:
- `database` - ORM and migrations
- `auth` - Authentication and authorization
- `queue` - Background job processing
- `cache` - Caching support
- `realtime` - WebSocket features
- `templates` - Server-side rendering
- `mail` - Email sending
- `storage` - File storage

## Common Patterns

### State Management

```rust
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
}

app.get("/", |State(state): State<AppState>| async move {
    // Use state.db
    Ok(Response::text("OK"))
});
```

### Error Handling

```rust
async fn handler() -> Result<ToxiResponse> {
    let data = fetch_data().await?;  // ? operator works
    Ok(Json(data))
}
```

## Troubleshooting

**Port already in use:**
```bash
# Change the port
Server::new(app).listen("127.0.0.1:8080".parse()?).await
```

**Dependency errors:**
```bash
cargo clean
cargo update
cargo build
```

## Resources

- [API Documentation](https://docs.rs/toxi)
- [GitHub Repository](https://github.com/Kyle6012/rust-toxi)
- [Example Applications](https://github.com/Kyle6012/rust-toxi/tree/main/examples)
