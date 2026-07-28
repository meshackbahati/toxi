# toxi-graphql

GraphQL integration for Toxi using Juniper.

## Installation

```toml
[dependencies]
toxi-graphql = "3.1.0"
```

## What This Crate Provides

- A default Juniper schema (`QueryRoot`, `MutationRoot`) via `create_schema()`
- `GraphQLHandler` for mounting GraphQL POST + playground GET endpoints
- `Context` with extension storage and optional database integration (`database` feature)

## Basic Usage

```rust
use toxi::prelude::*;
use toxi_graphql::create_handler;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().unwrap();
    let mut app = Application::new(config);

    // Mount at /graphql (POST for queries, GET for playground)
    create_handler().mount(app.router_mut())?;

    // or custom path
    // create_handler().mount_at(app.router_mut(), "/api/graphql")?;

    app.run().await
}
```

> **Alternative**: The manual approach using `Router::new()` → `create_handler().mount(&mut router)` → `Server::new(router).listen(addr)` works identically.

## Notes

- POST endpoint accepts both single and batch GraphQL requests.
- Context extensions can be used to pass request-scoped data to resolvers.
- The optional `database` feature enables `Context::with_database(...)`.
