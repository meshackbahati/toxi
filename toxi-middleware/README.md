# toxi-middleware

CORS, logging, compression, rate limiting, CSRF, security headers,
timeouts, and request IDs. Built on tower.

```toml
[dependencies]
toxi-middleware = "3"
```

```rust
use toxi::prelude::*;
use toxi_middleware::{ServiceBuilder, LoggerLayer, CorsLayer};

let service = ServiceBuilder::new()
    .layer(LoggerLayer::new())
    .layer(CorsLayer::permissive())
    .service(app.into_router());
Server::new(service).listen("127.0.0.1:3000".parse().unwrap()).await
```

Body-changing layers (CORS, compression) go through `ServiceBuilder`,
not `Router::layer`.
