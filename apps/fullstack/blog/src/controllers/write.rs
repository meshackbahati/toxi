//! Auth endpoints. Validation first, then the auth service.

use toxi::json_response;
use toxi::prelude::*;
use toxi_core::extract::Form;
use toxi_core::request::RequestExt;
use http_body_util::BodyExt;
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

/// GET /write — publish form.
pub async fn form(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let mut ctx = toxi_template::Context::new();
    ctx.set("title", "Write");
    let html = state
        .templates
        .render("write.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}

#[derive(serde::Deserialize)]
pub struct PublishForm {
    email: String,
    password: String,
    title: String,
    body: String,
}

/// POST /write — first publish creates the account, then the post.
pub async fn submit(
    State(state): State<Arc<AppState>>,
    Form(form): Form<PublishForm>,
) -> Result<Response> {
    crate::validators::credentials(&form.email, &form.password)?;
    crate::validators::post(&form.title, &form.body)?;

    // Login, or register on first publish.
    let user_id = match crate::services::auth::login(&state.db, &form.email, &form.password).await {
        Ok(id) => id,
        Err(_) => {
            let name = form.email.clone();
            crate::services::auth::register(&state.db, &form.email, &name, &form.password).await?
        }
    };
    crate::services::posts::insert(&state.db, &user_id, form.title.trim(), &form.body).await?;

    let res = http::Response::builder()
        .status(http::StatusCode::SEE_OTHER)
        .header(http::header::LOCATION, "/")
        .body(http_body_util::Full::new(bytes::Bytes::new()).map_err(|e| match e {}).boxed())
        .map_err(|e| Error::InternalServerError(format!("build: {e}")))?;
    Ok(toxi_core::ToxiResponse::new(res))
}
