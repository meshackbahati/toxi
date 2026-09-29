//! Attachment uploads stored under `uploads/`.
//!
//! Routes: POST /uploads?name=..., GET /uploads/:name.

use toxi::json_response;
use toxi_core::{Error, FromRequest, Request, Response, Result, State, ToxiResponse, extract::PathParams};
use http_body_util::BodyExt;
use std::sync::Arc;

use crate::AppState;

fn upload_dir() -> std::path::PathBuf {
    std::path::Path::new("uploads").to_path_buf()
}

fn safe_name(name: &str) -> Result<()> {
    let base = name.rsplit('/').next().unwrap_or(name);
    if base.is_empty() || base.contains("..") {
        return Err(Error::BadRequest("bad filename".to_string()));
    }
    Ok(())
}

/// Save the raw request body as a file. The name comes from `?name=`.
pub async fn upload(mut req: Request) -> Result<Response> {
    let _state: State<Arc<AppState>> = State::from_request(&mut req).await?;
    let query = req.uri().query().unwrap_or("").to_string();
    let raw = query
        .split('&')
        .find_map(|pair| pair.strip_prefix("name="))
        .unwrap_or("file");
    let stored = format!("{}-{}", uuid::Uuid::new_v4(), sanitize(raw));

    let bytes = req
        .body_mut()
        .collect()
        .await
        .map_err(|e| Error::InternalServerError(format!("body read: {e}")))?
        .to_bytes();
    if bytes.len() > 5 * 1024 * 1024 {
        return Err(Error::BadRequest("file over 5 MB".to_string()));
    }

    tokio::fs::create_dir_all(upload_dir())
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    tokio::fs::write(upload_dir().join(&stored), &bytes)
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    safe_name(&stored)?;
    Ok(json_response!({ "file": stored, "bytes": bytes.len() }))
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' { c } else { '_' })
        .take(64)
        .collect()
}

/// Serve a stored file.
pub async fn download(req: Request) -> Result<Response> {
    let name = req
        .extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("name"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    if name.contains('/') || name.contains("..") || name.is_empty() {
        return Err(Error::BadRequest("bad filename".to_string()));
    }
    let data = tokio::fs::read(upload_dir().join(&name))
        .await
        .map_err(|_| Error::NotFound("file not found".to_string()))?;
    let res = http::Response::builder()
        .header(http::header::CONTENT_TYPE, "application/octet-stream")
        .header(http::header::CONTENT_LENGTH, data.len())
        .body(http_body_util::Full::new(bytes::Bytes::from(data)).map_err(|e| match e {}).boxed())
        .map_err(|e| Error::InternalServerError(format!("build: {e}")))?;
    Ok(ToxiResponse::new(res))
}
