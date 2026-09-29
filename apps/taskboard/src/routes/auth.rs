//! Real authentication: argon-hashed passwords in sqlite plus JWT sessions.
//!
//! Routes: POST /auth/register, POST /auth/login, GET /auth/me.

use toxi::auth::{hash_password, verify_password, Claims};
use toxi::db::{sqlx, Database};
use toxi::json_response;
use toxi_core::{Error, FromRequest, Request, Response, Result, State};
use toxi_core::request::RequestExt;
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Read the bearer token from the request and verify it against app state.
pub async fn authenticate(req: &mut Request, state: &AppState) -> Result<Claims> {
    let header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let token = header.strip_prefix("Bearer ").unwrap_or("");
    if token.is_empty() {
        return Err(Error::Unauthorized("missing bearer token".to_string()));
    }
    state
        .jwt
        .verify(token)
        .map_err(|_| Error::Unauthorized("invalid token".to_string()))
}

/// Register a new user with a salted argon hash.
pub async fn register(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: RegisterRequest = req.json().await?;
    if body.password.len() < 8 {
        return Err(Error::BadRequest("password needs 8+ characters".to_string()));
    }

    let hash = hash_password(&body.password)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let user = crate::models::User::new(body.email.clone(), body.name.clone());
    let id = user.id.clone();
    state
        .db
        .execute_query(
            sqlx::query("INSERT INTO users (id, email, name, password_hash) VALUES ($1, $2, $3, $4)")
                .bind(&id)
                .bind(&body.email)
                .bind(&body.name)
                .bind(&hash),
        )
        .await
        .map_err(|e| Error::BadRequest(format!("register failed: {e}")))?;

    Ok(json_response!({ "id": id, "email": body.email }))
}

/// Verify credentials and return a signed JWT.
pub async fn login(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: LoginRequest = req.json().await?;

    let row = state
        .db
        .fetch_one(sqlx::query("SELECT id, password_hash FROM users WHERE email = $1").bind(&body.email))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::Unauthorized("unknown email".to_string()))?;

    use sqlx::Row;
    let id: String = row
        .try_get("id")
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let hash: String = row
        .try_get("password_hash")
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let ok = verify_password(&body.password, &hash)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    if !ok {
        return Err(Error::Unauthorized("wrong password".to_string()));
    }

    let token = state
        .jwt
        .generate_token(&Claims::new(id.clone(), 86400))
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "token": token, "id": id }))
}

/// Return the profile behind the bearer token.
pub async fn me(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let claims = authenticate(&mut req, &state).await?;

    let row = state
        .db
        .fetch_one(sqlx::query("SELECT id, email, name FROM users WHERE id = $1").bind(&claims.sub))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::Unauthorized("user gone".to_string()))?;

    use sqlx::Row;
    let id: String = row.try_get("id").unwrap_or_default();
    let email: String = row.try_get("email").unwrap_or_default();
    let name: String = row.try_get("name").unwrap_or_default();
    Ok(json_response!({ "id": id, "email": email, "name": name }))
}
