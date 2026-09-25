# toxi-openapi

OpenAPI 3.0 spec building with Swagger UI for Toxi.

```toml
[dependencies]
toxi-openapi = "3"
```

```rust
use toxi::prelude::*;
use toxi_openapi::{AutoDocs, OpenApiBuilder, PathItem, get_operation, Parameter, Response, Schema};

async fn list_users_handler(_req: Request) -> Result<Response> {
    Ok(Response::json([] as [String; 0]))
}

let list_users = get_operation("List users")
    .add_parameter(Parameter::query("page", Schema::integer()))
    .add_response("200", Response::json("ok", Schema::array(Schema::string())));

let spec = OpenApiBuilder::new("My API", "1.0.0")
    .path("/users", PathItem::default().with_get(list_users))
    .build();

let mut router = Router::new();
router.get("/users", list_users_handler);
let router = router.with_auto_docs(spec);
Server::new(router).listen("127.0.0.1:3000".parse().unwrap()).await
```

`with_auto_docs` serves the spec at `GET /openapi.json` and Swagger UI
at `GET /api/docs`. The spec is built by hand today: each path gets a
`PathItem` with operations, parameters, request bodies, and responses.
