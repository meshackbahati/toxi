//! Link endpoints: create, redirect (cache-first), list, stats.
//!
//! Routes: POST /links, GET /:code, GET /links, GET /stats.

use toxi::cache::Cache;
use toxi::db::{sqlx, Database};
use toxi::json_response;
use toxi_core::request::RequestExt;
use http_body_util::BodyExt;
use toxi_core::{Error, FromRequest, Request, Response, Result, State, extract::PathParams};
use std::sync::Arc;
use std::time::Duration;

use crate::models::{CreateLink, Link};
use crate::AppState;

fn code_of(req: &Request) -> String {
    req.extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("code"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn valid_code(code: &str) -> bool {
    !code.is_empty()
        && code.len() <= 32
        && code.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn valid_url(url: &str) -> bool {
    (url.starts_with("http://") || url.starts_with("https://")) && url.len() <= 2048
}

/// Create a short link, retrying on code collision.
pub async fn create(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: CreateLink = req.json().await?;
    if !valid_url(&body.url) {
        return Err(Error::BadRequest("url must be http(s), max 2048 chars".to_string()));
    }
    let wanted = body.code.unwrap_or_default();

    for _ in 0..5 {
        let code = if wanted.is_empty() {
            crate::models::random_code()
        } else {
            if !valid_code(&wanted) {
                return Err(Error::BadRequest("bad custom code".to_string()));
            }
            wanted.clone()
        };
        let done = state
            .db
            .execute_query(sqlx::query("INSERT INTO links (code, url) VALUES ($1, $2)").bind(&code).bind(&body.url))
            .await;
        match done {
            Ok(_) => {
                let _ = state.cache.set(&code, &body.url, Some(Duration::from_secs(3600))).await;
                return Ok(json_response!({ "code": code, "url": body.url }));
            }
            Err(_) => {
                if !wanted.is_empty() {
                    return Err(Error::Conflict("code taken".to_string()));
                }
            }
        }
    }
    Err(Error::InternalServerError("code collision, retry".to_string()))
}

/// Redirect to the target. Cache first, database on miss.
pub async fn redirect(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let code = code_of(&req);
    if !valid_code(&code) {
        return Err(Error::NotFound("unknown code".to_string()));
    }

    // Rate-limit hot redirects per caller.
    let caller = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    if !state.limiter.check(caller, "redirect").await {
        return Err(Error::RateLimited("slow down".to_string()));
    }

    if let Ok(Some(url)) = state.cache.get::<String>(&code).await {
        bump(&state, &code).await;
        return redirect_to(&url);
    }

    let row = state
        .db
        .fetch_one(sqlx::query("SELECT url FROM links WHERE code = $1").bind(&code))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::NotFound("unknown code".to_string()))?;
    use sqlx::Row;
    let url: String = row.try_get("url").map_err(|e| Error::InternalServerError(e.to_string()))?;
    let _ = state.cache.set(&code, &url, Some(Duration::from_secs(3600))).await;
    bump(&state, &code).await;
    redirect_to(&url)
}

async fn bump(state: &AppState, code: &str) {
    let _ = state
        .db
        .execute_query(sqlx::query("UPDATE links SET hits = hits + 1 WHERE code = $1").bind(code))
        .await;
}

fn redirect_to(url: &str) -> Result<Response> {
    let res = http::Response::builder()
        .status(http::StatusCode::FOUND)
        .header(http::header::LOCATION, url)
        .body(http_body_util::Full::new(bytes::Bytes::new()).map_err(|e| match e {}).boxed())
        .map_err(|e| Error::InternalServerError(format!("build: {e}")))?;
    Ok(toxi_core::ToxiResponse::new(res))
}

/// List links, newest first.
pub async fn list(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let rows = state
        .db
        .fetch_all(sqlx::query("SELECT code, url, hits FROM links ORDER BY created_at DESC LIMIT 100"))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    use sqlx::Row;
    let mut links = Vec::with_capacity(rows.len());
    for row in &rows {
        links.push(Link {
            code: row.try_get("code").unwrap_or_default(),
            url: row.try_get("url").unwrap_or_default(),
            hits: row.try_get("hits").unwrap_or(0),
        });
    }
    Ok(json_response!({ "links": links }))
}

/// Total links and hits.
pub async fn stats(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let row = state
        .db
        .fetch_one(sqlx::query("SELECT COUNT(*) AS n, COALESCE(SUM(hits), 0) AS h FROM links"))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    use sqlx::Row;
    let n: i64 = row.as_ref().and_then(|r| r.try_get("n").ok()).unwrap_or(0);
    let h: i64 = row.as_ref().and_then(|r| r.try_get("h").ok()).unwrap_or(0);
    let cached = state.cache.stats();
    Ok(json_response!({ "links": n, "hits": h, "cached": cached.hits }))
}
