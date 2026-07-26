# Migrating from Actix-web to Toxi

## Crate Comparison

| Actix-web | Toxi |
|-----------|------|
| `actix-web` | `toxi` |
| `actix-rt` | `tokio` |
| `actix-tera` | Built-in `TemplateContext` |
| `actix-cors` | Tower `CorsLayer` |

## Application State

```rust
// Actix-web
use actix_web::web;

struct AppState { db: sqlx::Pool<Sqlite> }

async fn handler(data: web::Data<AppState>) -> impl Responder {
    let _ = data.db.clone();
    HttpResponse::Ok().finish()
}
```

```rust
// Toxi
use std::sync::Arc;
use toxi::{State, Router};

struct AppState { db: sqlx::Pool<Sqlite> }

async fn handler(State(state): State<Arc<AppState>>) -> &'static str {
    let _ = state.db.clone();
    "ok"
}
```

## Routes & Handlers

```rust
// Actix-web
App::new()
    .route("/users/{id}", web::get().to(get_user))
    .route("/users", web::post().to(create_user))
```

```rust
// Toxi
Router::new()
    .get("/users/:id", get_user)
    .post("/users", create_user)
```

## Extractors

| Purpose | Actix-web | Toxi |
|---------|-----------|------|
| JSON body | `web::Json<T>` | `Json<T>` |
| Path params | `web::Path<T>` | `Path<T>` |
| Query string | `web::Query<T>` | `Query<T>` |
| Headers | `HttpRequest` + `.headers()` | `HeaderMap` |

## Middleware

```rust
// Actix-web — wrap_fn / wrap
App::new()
    .wrap(Logger::default())
    .wrap_fn(|req, srv| srv.call(req).map(|res| res))
```

```rust
// Toxi — Tower ServiceBuilder layers
use tower::ServiceBuilder;
use toxi::Router;

let mut app = Router::new();
app.layer(ServiceBuilder::new()
    .layer(tower_http::cors::CorsLayer::permissive())
    .into_inner());
```

## Template Rendering

```rust
// Actix-web (actix-tera)
async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    tmpl.render("index.html", &ctx).unwrap()
}
```

```rust
// Toxi
use toxi::template::TemplateContext;

async fn index(ctx: TemplateContext) -> String {
    ctx.render("index.html", &[("name", "world")])
}
```

## Database

```rust
// Actix-web — raw sqlx
async fn handler(pool: web::Data<sqlx::Pool<Sqlite>>) -> impl Responder {
    sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await;
}
```

```rust
// Toxi
use toxi::db::DbPool;

async fn handler(State(pool): State<Arc<DbPool>>) -> &'static str {
    sqlx::query("SELECT 1").fetch_one(pool.pool()).await;
    "ok"
}
```

## Request/Response

| Concept | Actix-web | Toxi |
|---------|-----------|------|
| Request | `HttpRequest` | `Request<Body>` |
| Response | `HttpResponse` | `Response` (alias over `http::Response<Body>`) |
| Error | `actix_web::Error` | `toxi::Error` |
| JSON return | `HttpResponse::Ok().json(&val)` | `Json(val)` as return |

## Key Notes

- Actix-web uses a trait-object-based middleware system; Toxi uses Tower's layer system (more composable).
- Toxi does not require `actix-rt` — just `#[tokio::main]`.
- Route param syntax: `{id}` (Actix) vs `:id` (Toxi).
- Toxi handlers return `impl IntoResponse` (similar to Axum), making it simpler.
