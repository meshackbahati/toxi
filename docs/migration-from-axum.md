# Migrating from Axum to Toxi

## Crate Comparison

| Axum | Toxi |
|------|------|
| `axum` | `toxi` |
| `axum::Router` | `toxi::Router` |
| `axum::extract::*` | `toxi::extract::*` |
| `axum::response::*` | `toxi::response::*` |
| No built-in templates | `toxi::template::TemplateContext` |
| No built-in auth | `toxi::auth` |
| No built-in ORM | `toxi::db` ORM & `DbPool` |

**Migration from Axum is the easiest — Toxi's API is directly inspired by Axum.**

## Router

```rust
// Axum
use axum::{Router, routing::get};

Router::new()
    .route("/", get(handler))
    .route("/users/:id", get(get_user))
```

```rust
// Toxi — near-identical syntax
use toxi::Router;

Router::new()
    .get("/", handler)
    .get("/users/:id", get_user)
```

## Extractors

```rust
// Axum
use axum::extract::{Path, Query, Json, State};
use std::sync::Arc;

async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Query(params): Query<HashMap<String, String>>,
    Json(body): Json<Payload>,
) -> Json<Value> {
    Json(json!({ "id": id }))
}
```

```rust
// Toxi — identical extractors
use toxi::extract::{Path, Query, Json, State};
use std::sync::Arc;

async fn handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Query(params): Query<HashMap<String, String>>,
    Json(body): Json<Payload>,
) -> Json<Value> {
    Json(json!({ "id": id }))
}
```

## State

Both use `State<Arc<AppState>>`. No structural difference — just change the import path.

```rust
// Axum
use axum::extract::State;
```

```rust
// Toxi
use toxi::extract::State;
```

## Middleware

Both share the Tower ecosystem. Code is portable as-is.

```rust
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// Works in both Axum and Toxi — identical API
let mut app = Router::new();
app.layer(ServiceBuilder::new()
    .layer(CorsLayer::permissive())
    .layer(TraceLayer::new_for_http())
    .into_inner());
```

## Templates (New in Toxi)

Axum has no built-in templating. In Toxi, you get it out of the box:

```rust
// Axum — you needed extra crates (minijinja, tera, handlebars)
// Toxi — built-in
use toxi::template::TemplateContext;

async fn index(ctx: TemplateContext) -> String {
    ctx.render("index.html", &[("title", "Home")])
}
```

## Database (New in Toxi)

```rust
// Axum — manual pool setup and state management
use sqlx::PgPool;

let pool = PgPool::connect("...").await.unwrap();
let app = Router::new().with_state(Arc::new(AppState { pool }));
```

```rust
// Toxi — built-in DbPool with ORM support
use toxi::db::DbPool;

let pool = DbPool::connect("sqlite://db.sqlite").await.unwrap();
let app = Router::new()
    .get("/", handler)
    .with_state(pool);  // DbPool directly usable as state
```

## Response Types

| Axum | Toxi |
|------|------|
| `impl IntoResponse` | `impl IntoResponse` |
| `Json(val)` | `Json(val)` |
| `Html(val)` | `Html(val)` |
| `StatusCode` | `StatusCode` |
| `(StatusCode, Json<T>)` | `(StatusCode, Json<T>)` |
| `Response` | `Response` (both built on `http::Response`) |

## What You Gain

| Feature | Axum | Toxi |
|---------|------|------|
| Templating | External crate | Built-in `TemplateContext` |
| Auth / JWT | External crate | Built-in `toxi::auth` |
| ORM / Migrations | External crate | Built-in `toxi::db` with derive macros |
| CLI scaffolding | None | `cargo toxi new` |
| Config (TOML) | Manual | `toxi.toml` auto-loaded |

## Migration Steps

1. Replace `axum::` imports with `toxi::` (e.g. `axum::Router` → `toxi::Router`).
2. Change `Router::new().route("/", get(fn))` to `Router::new().get("/", fn)`.
3. Wrap application state in `Arc` if not already done.
4. Optionally adopt `DbPool` instead of raw sqlx pool.
5. Add templates or auth using Toxi's built-in modules.
