//! Auth endpoints. Validation first, then the auth service.

use toxi::json_response;
use toxi::prelude::*;
use toxi_core::request::RequestExt;
use std::sync::Arc;

use crate::services::auth;
use crate::validators;
use crate::AppState;

#[derive(serde::Deserialize)]
struct Credentials {
    email: String,
    password: String,
    name: Option<String>,
}

fn bearer(req: &Request) -> String {
    req.headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .strip_prefix("Bearer ")
        .unwrap_or("")
        .to_string()
}

/// POST /auth/register — create an author.
pub async fn register(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: Credentials = req.json().await?;
    validators::credentials(&body.email, &body.password)?;
    let name = body.name.unwrap_or_else(|| body.email.clone());
    let id = auth::register(&state.db, &body.email, &name, &body.password).await?;
    Ok(json_response!({ "id": id }))
}

/// POST /auth/login — return a JWT.
pub async fn login(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: Credentials = req.json().await?;
    validators::credentials(&body.email, &body.password)?;
    let id = auth::login(&state.db, &body.email, &body.password).await?;
    let token = auth::mint(&state.jwt, &id)?;
    Ok(json_response!({ "token": token }))
}

/// POST /posts — publish as the token owner.
pub async fn publish(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let token = bearer(&req);
    if token.is_empty() {
        return Err(Error::Unauthorized("missing bearer token".to_string()));
    }
    let user_id = auth::authorize(&state.jwt, &token)?;
    let body: crate::models::WritePost = req.json().await?;
    validators::post(&body.title, &body.body)?;
    let id = crate::services::posts::insert(&state.db, &user_id, body.title.trim(), &body.body).await?;
    Ok(json_response!({ "id": id }))
}
