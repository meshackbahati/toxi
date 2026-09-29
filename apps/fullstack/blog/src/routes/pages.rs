//! Rendered pages: index lists posts, each post gets a page.

use toxi::prelude::*;
use toxi::db::{sqlx, Database};
use toxi_core::extract::PathParams;
use toxi_template::Context;
use std::sync::Arc;

use crate::models::Post;
use crate::AppState;

/// GET / — index of all posts, newest first.
pub async fn index(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let posts = latest(&state, 50).await?;
    let mut ctx = Context::new();
    ctx.set("title", "Blog");
    ctx.set("posts", posts);
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
    let post = one(&state, &id)
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

pub async fn latest(state: &AppState, limit: i64) -> Result<Vec<Post>> {
    use sqlx::Row;
    let rows = state
        .db
        .fetch_all(sqlx::query(
            "SELECT p.id, p.user_id, u.name AS author, p.title, p.body
             FROM posts p JOIN users u ON u.id = p.user_id
             ORDER BY p.created_at DESC LIMIT $1",
        ).bind(limit))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let mut posts = Vec::with_capacity(rows.len());
    for row in &rows {
        posts.push(Post {
            id: row.try_get("id").unwrap_or_default(),
            user_id: row.try_get("user_id").unwrap_or_default(),
            author: row.try_get("author").unwrap_or_default(),
            title: row.try_get("title").unwrap_or_default(),
            body: row.try_get("body").unwrap_or_default(),
        });
    }
    Ok(posts)
}

async fn one(state: &AppState, id: &str) -> Result<Option<Post>> {
    use sqlx::Row;
    let row = state
        .db
        .fetch_one(sqlx::query(
            "SELECT p.id, p.user_id, u.name AS author, p.title, p.body
             FROM posts p JOIN users u ON u.id = p.user_id WHERE p.id = $1",
        ).bind(id))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(row.map(|row| Post {
        id: row.try_get("id").unwrap_or_default(),
        user_id: row.try_get("user_id").unwrap_or_default(),
        author: row.try_get("author").unwrap_or_default(),
        title: row.try_get("title").unwrap_or_default(),
        body: row.try_get("body").unwrap_or_default(),
    }))
}
