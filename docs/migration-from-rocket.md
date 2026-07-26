# Migrating from Rocket to Toxi

## Crate Comparison

| Rocket | Toxi |
|--------|------|
| `rocket` | `toxi` |
| `rocket_dyn_templates` | Built-in `TemplateContext` |
| `rocket::fairing` | Tower middleware layers |
| `rocket::serde::json::Json` | `toxi::extract::Json` |

## Routing

```rust
// Rocket — attribute-based
#[get("/users/<id>")]
fn get_user(id: u64) -> Json<User> { ... }

#[post("/users", data = "<user>")]
fn create_user(user: Json<NewUser>) -> Json<User> { ... }
```

```rust
// Toxi — method-chaining
Router::new()
    .get("/users/:id", get_user)
    .post("/users", create_user)
```

## State

```rust
// Rocket
use std::sync::Arc;

struct AppState { pool: sqlx::PgPool }

#[get("/")]
fn index(state: &State<Arc<AppState>>) -> String {
    format!("Pool: {}", state.pool)
}
```

```rust
// Toxi
use toxi::{State, Router};
use std::sync::Arc;

struct AppState { pool: sqlx::PgPool }

async fn index(State(state): State<Arc<AppState>>) -> String {
    format!("Pool: {}", state.pool)
}
```

## Data Extractors

```rust
// Rocket
#[post("/form")]
fn submit(form: Form<FormData>) -> String { ... }

#[post("/json")]
fn submit_json(data: Json<Payload>) -> String { ... }
```

```rust
// Toxi — nearly identical
Router::new()
    .post("/form", |form: Form<FormData>| async { "ok" })
    .post("/json", |data: Json<Payload>| async { "ok" });
```

## Templates

```rust
// Rocket (rocket_dyn_templates)
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { name: "world" })
}
```

```rust
// Toxi
use toxi::template::TemplateContext;

async fn index(ctx: TemplateContext) -> String {
    ctx.render("index", &[("name", "world")])
}
```

## Error Handling / Catchers

```rust
// Rocket
#[catch(404)]
fn not_found() -> String {
    "Resource not found".to_string()
}

rocket::build().register("/", catchers![not_found])
```

```rust
// Toxi — early return with framework errors
async fn handler() -> Result<&'static str, toxi::Error> {
    Err(toxi::Error::NotFound("Resource not found".into()))
}
```

## Fairings → Middleware

```rust
// Rocket
struct Logger;
#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info { Info { name: "Logger", kind: Kind::Request } }
    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) { }
}
```

```rust
// Toxi — Tower layer
use tower_http::trace::TraceLayer;

Router::new()
    .get("/", handler)
    .layer(TraceLayer::new_for_http())
```

## Launch

```rust
// Rocket
#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
```

```rust
// Toxi
#[tokio::main]
async fn main() {
    let app = Router::new().get("/", index);
    toxi::serve(app, "0.0.0.0:3000").await.unwrap();
}
```

## Key Differences

| Rocket | Toxi |
|--------|------|
| Attribute-based routing (`#[get]`, `#[post]`) | Method-chaining (`router.get()`, `.post()`) |
| `&State<T>` | `State<Arc<T>>` |
| `#[catch]` for error handling | `Error::NotFound`, `Error::BadRequest` etc. |
| Fairings system | Tower middleware (Layered) |
| `#[launch]` / `#[rocket::main]` | `#[tokio::main]` |
| `context!` macro for templates | `(&[("key", "val")])` tuples |
| Managed pool via Rocket state | `DbPool` wrapper or raw sqlx |

## Notes

- Rocket uses `{id}` or `<id>` in routes; Toxi uses `:id`.
- Toxi handlers must be `async fn`; Rocket supports sync handlers.
- Rocket's form/query/JSON extractors map directly to Toxi equivalents.
- No need for Rocket's `build` / `launch` ceremony — just `toxi::serve`.
