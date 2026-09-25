# toxi-graphql

GraphQL endpoint (Juniper) that mounts on a Toxi router.

```toml
[dependencies]
toxi-graphql = "3"
```

```rust
use toxi::prelude::*;
use toxi_graphql::create_handler;

let mut app = Application::new(Config::load().unwrap());
create_handler().mount(app.router_mut())?;
app.run().await
```
