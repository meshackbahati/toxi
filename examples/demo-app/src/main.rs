use toxi::prelude::*;
use toxi_template::{TemplateContext, Context};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    templates: Arc<TemplateContext>,
    visit_count: Arc<std::sync::atomic::AtomicU64>,
}

impl AppState {
    fn new() -> Result<Self> {
        let templates = TemplateContext::new("examples/demo-app/templates");
        Ok(Self {
            templates: Arc::new(templates),
            visit_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        })
    }
}

async fn home(state: State<Arc<AppState>>) -> Result<Response> {
    let count = state.0.visit_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

    let mut ctx = Context::new();
    ctx.set("title", "Toxi Demo App");
    ctx.set("welcome_message", "Welcome to Toxi v3 Demo");
    ctx.set("visit_count", count);
    ctx.set("features", vec![
        "Type-safe Request Handling",
        "Multiple Response Types",
        "Template Engine with Fullstack Support",
        "TemplateContext + load_dir() from templates/ folder",
        "Layout Inheritance with blocks",
        "Path Parameters",
        "Query Parameters",
        "JSON Body Parsing",
        "Cookie Handling",
        "Comprehensive Error Handling",
    ]);

    let html = state.0.templates.render("home.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}

async fn api_docs(state: State<Arc<AppState>>) -> Result<Response> {
    let ctx = Context::from_json(serde_json::json!({
        "spec_url": "/api/openapi.json"
    }));
    let html = state.0.templates.render("api_docs.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}

async fn api_status(_req: Request) -> Result<Response> {
    Ok(Response::json(serde_json::json!({
        "status": "online",
        "framework": "Toxi",
        "version": "3.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
}

async fn health_check(_req: Request) -> Result<Response> {
    Ok(Response::text("OK"))
}

async fn ping(_req: Request) -> Result<Response> {
    Ok(Response::ok())
}

async fn get_users(Query(params): Query<serde_json::Value>) -> Result<Response> {
    let page = params.get("page").and_then(|v| v.as_u64()).unwrap_or(1);
    let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(10);
    let offset = (page - 1) * limit;

    let users: Vec<_> = ((offset + 1)..=(offset + limit))
        .map(|i| serde_json::json!({
            "id": i,
            "name": format!("User {}", i),
            "email": format!("user{}@example.com", i),
            "active": true,
        }))
        .collect();

    Ok(Response::json(serde_json::json!({
        "users": users,
        "pagination": { "page": page, "limit": limit, "offset": offset, "total": 100 },
    })))
}

async fn get_user_detail(Path(user_id): Path<u32>) -> Result<Response> {
    Ok(Response::json(serde_json::json!({
        "user": {
            "id": user_id,
            "name": format!("User {}", user_id),
            "email": format!("user{}@example.com", user_id),
            "joined_date": chrono::Utc::now().date_naive(),
            "active": true,
        }
    })))
}

#[derive(serde::Deserialize, Debug)]
struct UserFormData {
    name: String,
    email: String,
    message: String,
}

async fn submit_form(Json(form_data): Json<UserFormData>) -> Result<Response> {
    Ok(Response::json(serde_json::json!({
        "status": "success",
        "message": "Form submitted successfully",
        "received_data": {
            "name": form_data.name,
            "email": form_data.email,
            "message_length": form_data.message.len(),
        }
    })))
}

async fn handle_cookies(cookies: Cookies) -> Result<Response> {
    let mut cookies_map = serde_json::Map::new();
    let mut count = 0;
    for (name, value) in cookies.iter() {
        cookies_map.insert(name.to_string(), serde_json::Value::String(value.to_string()));
        count += 1;
    }
    Ok(Response::json(serde_json::json!({
        "message": "Cookie information retrieved",
        "cookie_count": count,
        "cookies": cookies_map,
    })))
}

async fn error_example(Query(params): Query<serde_json::Value>) -> Result<Response> {
    if let Some(error_type) = params.get("type").and_then(|v| v.as_str()) {
        match error_type {
            "not_found" => Err(Error::NotFound("Resource not found".to_string())),
            "bad_request" => Err(Error::BadRequest("Bad request example".to_string())),
            "unauthorized" => Err(Error::Unauthorized("Unauthorized access".to_string())),
            "forbidden" => Err(Error::Forbidden("Access forbidden".to_string())),
            "conflict" => Err(Error::Conflict("Resource conflict".to_string())),
            "validation" => Err(Error::Validation("Validation failed".to_string())),
            "rate_limited" => Err(Error::RateLimited("Rate limit exceeded".to_string())),
            "service_unavailable" => Err(Error::ServiceUnavailable("Service temporarily unavailable".to_string())),
            _ => Ok(Response::json(json!({ "status": "unknown_error_type" }))),
        }
    } else {
        Ok(Response::json(json!({ "status": "no_error_requested" })))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState::new()
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let state = Arc::new(state);

    let mut router = Router::new();
    let state_clone = state.clone();

    router.get("/", move |_: Request| home(State(state_clone.clone())));
    router.get("/api-docs", {
        let s = state.clone();
        move |_: Request| api_docs(State(s.clone()))
    });
    router.get("/api/status", api_status);
    router.get("/users", get_users);
    router.get("/users/:id", get_user_detail);
    router.post("/submit", submit_form);
    router.get("/cookies", handle_cookies);
    router.get("/error", error_example);
    router.get("/health", health_check);
    router.get("/ping", ping);
    let server = Server::new(router);
    println!("Toxi v3 Demo App running on http://127.0.0.1:3000");
    println!();
    println!("Available Endpoints:");
    println!("  GET  /              - Home page (TemplateContext + HTML)");
    println!("  GET  /api-docs      - API docs (TemplateContext + HTML)");
    println!("  GET  /api/status    - API status (JSON)");
    println!("  GET  /users         - Users list (JSON)");
    println!("  GET  /users/:id     - User detail (JSON)");
    println!("  POST /submit        - Form submission (JSON body)");
    println!("  GET  /cookies       - Cookie handling (JSON)");
    println!("  GET  /error         - Error examples");
    println!("  GET  /health        - Health check (text)");
    println!("  GET  /ping          - Ping (empty response)");
    println!();
    println!("Fullstack Pattern:");
    println!("  Templates loaded from: examples/demo-app/templates/");
    println!("  Using TemplateContext::new() + render() with layout inheritance");

    server.listen("127.0.0.1:3000".parse().unwrap()).await
}
