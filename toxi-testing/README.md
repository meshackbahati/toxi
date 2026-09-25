# toxi-testing

Request builders and a test server for Toxi handlers.

```toml
[dev-dependencies]
toxi-testing = "3"
```

```rust
use toxi::prelude::*;
use toxi_testing::{test_router, TestRequest};

async fn ping(_req: Request) -> Result<Response> {
    Ok(Response::text("pong"))
}

#[tokio::test]
async fn ping_route_responds() {
    let mut router = Router::new();
    router.get("/ping", ping);
    let mut server = test_router(router);
    let resp = server.call(TestRequest::get("/ping").build_toxi()).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);
}
```
