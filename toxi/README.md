# toxi

The full Toxi web framework in one crate. Routing, ORM, auth,
templates, WebSockets, queues, cache, storage, mail, and OpenAPI docs.
Each piece also lives in its own crate if you want less.

```toml
[dependencies]
toxi = "3"
```

```rust
use toxi::prelude::*;

async fn hello(_req: Request) -> Result<Response> {
    Ok(Response::text("Hello, Toxi!"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new(Config::load().unwrap());
    app.router_mut().get("/", hello);
    app.run().await
}
```

Crates: `toxi-core`, `toxi-db`, `toxi-auth`, `toxi-realtime`,
`toxi-queue`, `toxi-cache`, `toxi-storage`, `toxi-template`,
`toxi-openapi`, `toxi-graphql`, `toxi-middleware`, `toxi-security`,
`toxi-utils`, `toxi-plugin`, `toxi-macros`, `toxi-cli`, `toxi-config`,
`toxi-testing`, `toxi-mail`.
