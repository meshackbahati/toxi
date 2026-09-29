//! Rendered pages. Thin by design: templates plus service calls.

use toxi::prelude::*;
use toxi_core::extract::PathParams;
use toxi_template::Context;
use std::sync::Arc;

use crate::services::posts;
use crate::AppState;

/// GET / — index of all posts, newest first.
pub async fn index(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let list = posts::latest(&state.db, 50).await?;
    let mut ctx = Context::new();
    ctx.set("title", "Blog");
    ctx.set("posts", list);
    ctx.set("post_count", posts::count(&state.db).await?);
    let html = state
        .templates
        .render("index.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}

/// GET /posts/:id — one rendered post.
pub async fn show(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let id = req
        .extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let post = posts::one(&state.db, &id)
        .await?
        .ok_or_else(|| Error::NotFound("post not found".to_string()))?;
    let mut ctx = Context::new();
    ctx.set("title", post.title.clone());
    ctx.set("post", post);
    let html = state
        .templates
        .render("post.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}
