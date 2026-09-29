//! Auth-gated writing: register, login, publish.
//!
//! Routes: POST /auth/register, POST /auth/login, POST /posts.

use toxi::auth::{Claims, hash_password, verify_password};
use toxi::db::{sqlx, Database};
use toxi::json_response;
use toxi::prelude::*;
use toxi_core::request::RequestExt;
use std::sync::Arc;

use crate::models::WritePost;
use crate::AppState;

#[derive(serde::Deserialize)]
struct Credentials {
    email: String,
    password: String,
    name: Option<String>,
}

fn bearer(req: &Request) -> Result<String> {
    Ok(req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .strip_prefix("Bearer ")
        .unwrap_or("")
        .to_string())
}

/// POST /auth/register — create an author.
pub async fn register(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: Credentials = req.json().await?;
    let name = body.name.unwrap_or_else(|| body.email.clone());
    if body.password.len() < 8 {
        return Err(Error::BadRequest("password needs 8+ characters".to_string()));
    }
    let hash = hash_password(&body.password)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let id = uuid::Uuid::new_v4().to_string();
    state
        .db
        .execute_query(
            sqlx::query("INSERT INTO users (id, email, name, password_hash) VALUES ($1, $2, $3, $4)")
                .bind(&id)
                .bind(&body.email)
                .bind(&name)
                .bind(&hash),
        )
        .await
        .map_err(|e| Error::BadRequest(format!("register failed: {e}")))?;
    Ok(json_response!({ "id": id }))
}

/// POST /auth/login — return a JWT.
pub async fn login(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: Credentials = req.json().await?;
    let row = state
        .db
        .fetch_one(sqlx::query("SELECT id, password_hash FROM users WHERE email = $1").bind(&body.email))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::Unauthorized("unknown email".to_string()))?;
    use sqlx::Row;
    let id: String = row.try_get("id").map_err(|e| Error::InternalServerError(e.to_string()))?;
    let hash: String = row
        .try_get("password_hash")
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    if !verify_password(&body.password, &hash).map_err(|e| Error::InternalServerError(e.to_string()))? {
        return Err(Error::Unauthorized("wrong password".to_string()));
    }
    let token = state
        .jwt
        .generate_token(&Claims::new(id, 86400))
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "token": token }))
}

/// Resolve the bearer token to a user id.
pub async fn author_id(req: &Request, state: &AppState) -> Result<String> {
    let token = bearer(req)?;
    if token.is_empty() {
        return Err(Error::Unauthorized("missing bearer token".to_string()));
    }
    state
        .jwt
        .verify(&token)
        .map(|c| c.sub)
        .map_err(|_| Error::Unauthorized("invalid token".to_string()))
}

/// POST /posts — publish as the token owner.
pub async fn publish(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let user_id = author_id(&req, &state).await?;
    let body: WritePost = req.json().await?;
    if body.title.trim().is_empty() {
        return Err(Error::BadRequest("title is empty".to_string()));
    }
    let id = uuid::Uuid::new_v4().to_string();
    state
        .db
        .execute_query(
            sqlx::query("INSERT INTO posts (id, user_id, title, body) VALUES ($1, $2, $3, $4)")
                .bind(&id)
                .bind(&user_id)
                .bind(body.title.trim())
                .bind(&body.body),
        )
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "id": id }))
}
