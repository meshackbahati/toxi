//! Paste endpoints: create stores raw plus sanitized, view renders.
//!
//! Routes: GET/POST /pastes, GET /pastes/:id, GET /pastes/:id/raw.

use toxi::db::{sqlx, Database};
use toxi::json_response;
use toxi::security::{escape_html, sanitize_html};
use toxi::prelude::*;
use toxi_core::extract::PathParams;
use toxi_core::request::RequestExt;
use toxi_template::Context;
use std::sync::Arc;

use crate::models::{CreatePaste, Paste};
use crate::AppState;

fn id_of(req: &Request) -> String {
    req.extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn row_paste(row: &sqlx::any::AnyRow) -> Result<Paste> {
    use sqlx::Row;
    let db_err = |e: sqlx::Error| Error::InternalServerError(e.to_string());
    let body: String = row.try_get("body").map_err(db_err)?;
    Ok(Paste {
        id: row.try_get("id").map_err(db_err)?,
        title: row.try_get("title").map_err(db_err)?,
        preview: sanitize_html(&body),
        body,
    })
}

/// GET /pastes — newest first, with previews.
pub async fn list(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let rows = state
        .db
        .fetch_all(sqlx::query("SELECT id, title, body FROM pastes ORDER BY created_at DESC LIMIT 100"))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let mut pastes = Vec::with_capacity(rows.len());
    for row in &rows {
        pastes.push(row_paste(row)?);
    }
    Ok(json_response!({ "pastes": pastes }))
}

/// POST /pastes — store raw, serve sanitized.
pub async fn create(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: CreatePaste = req.json().await?;
    if body.body.trim().is_empty() {
        return Err(Error::BadRequest("body is empty".to_string()));
    }
    let paste = Paste {
        id: uuid::Uuid::new_v4().to_string(),
        title: body.title.unwrap_or_default(),
        preview: sanitize_html(&body.body),
        body: body.body,
    };
    state
        .db
        .execute_query(
            sqlx::query("INSERT INTO pastes (id, title, body) VALUES ($1, $2, $3)")
                .bind(&paste.id)
                .bind(&paste.title)
                .bind(&paste.body),
        )
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "paste": paste }))
}

/// GET /pastes/:id — rendered page with the sanitized preview.
pub async fn view(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let paste = one(&state, &id_of(&req)).await?;
    let mut ctx = Context::new();
    ctx.set("title", paste.title.clone());
    ctx.set("preview", paste.preview.clone());
    ctx.set("id", paste.id.clone());
    let html = state
        .templates
        .render("paste.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}

/// GET /pastes/:id/raw — the original bytes, HTML-escaped.
pub async fn raw(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let paste = one(&state, &id_of(&req)).await?;
    Ok(Response::text(escape_html(&paste.body)))
}

async fn one(state: &AppState, id: &str) -> Result<Paste> {
    let row = state
        .db
        .fetch_one(sqlx::query("SELECT id, title, body FROM pastes WHERE id = $1").bind(id))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::NotFound("paste not found".to_string()))?;
    row_paste(&row)
}
