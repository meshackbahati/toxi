use toxi::prelude::*;
use toxi_template::{TemplateContext, Context};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    templates: Arc<TemplateContext>,
}

async fn home(state: State<Arc<AppState>>) -> Result<Response> {
    let mut ctx = Context::new();
    ctx.set("title", "Toxi Demo - Fullstack Pattern");
    ctx.set("heading", "TemplateContext + load_dir() Demo");
    ctx.set("content", "This page is rendered using TemplateContext which loads templates from the templates/ directory. Each request creates a fresh TemplateEngine internally, keeping the server runtime decoupled from presentation failures.");
    ctx.set("items", vec![
        "Templates stored as separate files in templates/",
        "TemplateContext::new(\"path\") for shared state",
        "State<Arc<TemplateContext>> extractor in handlers",
        "Layout inheritance with {% extends %}",
        "Auto-escaping with {{ safe }} filter",
    ]);

    let html = state.0.templates.render("page.html", &ctx)
        .map_err(|e| Error::Server(e.to_string()))?;
    Ok(Response::html(html))
}

async fn api_endpoint(_req: Request) -> Result<Response> {
    Ok(Response::json(serde_json::json!({
        "message": "Hello from API",
        "framework": "Toxi",
        "version": "3.0.0",
        "features": [
            "TemplateContext with load_dir()",
            "Arc<TemplateContext> shared state",
            "Server-side rendering",
        ]
    })))
}

async fn text_response(_req: Request) -> Result<Response> {
    Ok(Response::text("This is a plain text response from Toxi v3!"))
}

async fn empty_response(_req: Request) -> Result<Response> {
    Ok(Response::ok())
}

async fn user_detail(Path(user_id): Path<u32>) -> Result<Response> {
    Ok(Response::json(serde_json::json!({
        "user": {
            "id": user_id,
            "name": format!("User {}", user_id),
            "email": format!("user{}@example.com", user_id),
        }
    })))
}

async fn error_handler(Query(params): Query<serde_json::Value>) -> Result<Response> {
    if let Some(error_type) = params.get("type").and_then(|v| v.as_str()) {
        match error_type {
            "not_found" => Err(Error::NotFound("Resource not found".to_string())),
            "bad_request" => Err(Error::BadRequest("Bad request example".to_string())),
            _ => Ok(Response::json(serde_json::json!({ "status": "unknown_error" }))),
        }
    } else {
        Ok(Response::json(serde_json::json!({ "status": "no_error" })))
    }
}

async fn template_demo(state: State<Arc<AppState>>) -> Result<Response> {
    let mut ctx = Context::new();
    ctx.set("title", "Template Demo");
    ctx.set("heading", "Template Engine Example");
    ctx.set("content", "This page was rendered using Toxi template engine with templates loaded from the templates directory.");
    ctx.set("items", vec!["Template loading from directory", "Layout inheritance", "Variable interpolation"]);

    let html = state.0.templates.render("page.html", &ctx)
        .map_err(|e| Error::Server(e.to_string()))?;
    Ok(Response::html(html))
}

#[tokio::main]
async fn main() -> Result<()> {
    let templates = TemplateContext::new("examples/demo-app/templates");
    let state = Arc::new(AppState { templates });

    let mut router = Router::new();
    let s = state.clone();
    router.get("/", move |_: Request| home(State(s.clone())));
    router.get("/api", api_endpoint);
    router.get("/text", text_response);
    router.get("/empty", empty_response);
    router.get("/users/:id", user_detail);
    router.get("/error", error_handler);
    {
        let s = state.clone();
        router.get("/template", move |_: Request| template_demo(State(s.clone())));
    }

    let server = Server::new(router);
    println!("Toxi v3 Demo running on http://127.0.0.1:3000");
    println!("Templates loaded from: examples/demo-app/templates/");
    println!("Using TemplateContext::new() + load_dir() pattern");

    server.listen("127.0.0.1:3000".parse().unwrap()).await
}
