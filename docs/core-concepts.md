# Core Concepts

This document explains the fundamental concepts of the Toxi web framework.

## Architecture Overview

Toxi follows a modular architecture with the following main components:

- **toxi-core**: HTTP server, routing, and basic request/response handling
- **toxi-db**: Database ORM and migrations
- **toxi-auth**: Authentication and authorization
- **toxi-middleware**: Middleware components
- **toxi-template**: Template engine
- **toxi-cli**: Command-line tools

## Request-Response Lifecycle

The typical lifecycle of a request in Toxi:

1. **Incoming Request**: HTTP request arrives at the server
2. **Routing**: Router matches the path to a handler function
3. **Middleware Processing**: Request passes through configured middleware layers
4. **Handler Execution**: Handler function processes the request
5. **Response Creation**: Handler returns a response
6. **Middleware Processing**: Response passes back through middleware
7. **Response Sent**: Server sends response back to client

## Router

The Router is responsible for mapping incoming HTTP requests to handler functions.

### Creating a Router

```rust
use toxi::prelude::*;

let mut router = Router::new();
```

### Adding Routes

```rust
// Different HTTP methods
router.get("/users", list_users);
router.post("/users", create_user);
router.put("/users/:id", update_user);
router.delete("/users/:id", delete_user);
router.patch("/users/:id", partial_update);

// Path parameters (captured and available in handlers)
router.get("/users/:id", get_user);
router.get("/users/:user_id/posts/:post_id", get_post);

// Wildcards (match any path)
router.get("/static/*", serve_static);
```

### Route Matching Priority

Routes are matched in the order they are registered. More specific routes should be registered before general ones:

```rust
// More specific route first
router.get("/users/me", get_current_user);
// More general route after
router.get("/users/:id", get_user);
```

## Handlers

Handlers are async functions that process requests and return responses.

### Handler Signature

```rust
use toxi::prelude::*;

async fn handler(request: Request) -> Result<Response> {
    // Process request
    Ok(Response::text("Hello, World!"))
}
```

### Handler with Extractors

```rust
use toxi::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct UserId {
    id: u64,
}

async fn get_user(
    Path(params): Path<UserId>
) -> Result<Response> {
    Ok(json_response!({
        "id": params.id,
        "name": "User Name"
    }))
}
```

## Request and Response Types

### Request

Represents an incoming HTTP request with:

- HTTP method
- URI (path and query)
- Headers
- Body
- Extensions (for storing additional data like path params, state)

### Response

Represents an outgoing HTTP response with:

- Status code
- Headers
- Body

## Response Utilities

Toxi provides utility functions to create common response types:

```rust
use toxi::response;

// JSON response
let json_resp = json_response!({"key": "value"});

// HTML response
let html_resp = Response::html("<h1>Hello</h1>");

// Text response
let text_resp = Response::text("Plain text");
```

## Extractors

Extractors are types that implement the `FromRequest` trait to extract data from requests.

### Available Extractors

- `Json<T>`: Extracts and deserializes JSON from request body
- `Query<T>`: Extracts and deserializes query parameters
- `Path<T>`: Extracts and deserializes path parameters
- `State<T>`: Extracts application state from request extensions

### Json Extractor

```rust
use toxi::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(Json(data): Json<CreateUser>) -> Result<Response> {
    Ok(json_response!({
        "id": 1,
        "name": data.name,
        "email": data.email
    }))
}
```

### Query Extractor

```rust
use toxi::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_items(Query(params): Query<Pagination>) -> Result<Response> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    
    Ok(json_response!({
        "page": page,
        "limit": limit
    }))
}
```

### Path Extractor

```rust
use toxi::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct UserId {
    id: u64,
}

async fn get_user(Path(params): Path<UserId>) -> Result<Response> {
    Ok(json_response!({
        "id": params.id
    }))
}
```

### State Extractor

```rust
use toxi::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    app_name: String,
}

async fn handler(State(state): State<Arc<AppState>>) -> Result<Response> {
    Ok(json_response!({
        "app_name": state.app_name
    }))
}
```

## Application State

To share state across handlers, use the State extractor:

```rust
use toxi::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
    config: Config,
}

// Inject state into router
let state = Arc::new(AppState { /* ... */ });

let router = router.with_state(state);
```

Then extract it in handlers:

```rust
async fn handler(State(state): State<Arc<AppState>>) -> Result<Response> {
    // Use state.db_pool, state.config, etc.
    Ok(Response::text("Success"))
}
```

## Error Handling

Toxi uses a Result-based error handling system:

```rust
use toxi::prelude::*;

// Error variants in toxi-core
enum Error {
    Server(String),
    NotFound,
    BadRequest(String),
    Unauthorized(String),
    Hyper(hyper::Error),
    Io(std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

// In handlers
async fn handler(_req: Request) -> Result<Response> {
    // This will return a Server error if it fails
    let data = some_operation_that_might_fail()?;
    
    Ok(json_response!(data))
}
```

## Server Configuration

The Server component listens on a socket and handles incoming connections:

```rust
use toxi::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = Router::new();
    router.get("/", handler);
    
    Server::new(router)
        .listen("127.0.0.1:3000".parse().unwrap())
        .await
}
```

The server creates a new task for each incoming connection, enabling concurrent request handling.