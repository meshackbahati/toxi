//! Credential handling: registration, login, token checks.

use toxi::auth::{Claims, JwtManager, hash_password, verify_password};
use toxi::db::{sqlx, Database, DbPool};
use toxi_core::{Error, Result};

/// Register an author, returning the new user id.
pub async fn register(db: &DbPool, email: &str, name: &str, password: &str) -> Result<String> {
    let hash =
        hash_password(password).map_err(|e| Error::InternalServerError(e.to_string()))?;
    let id = uuid::Uuid::new_v4().to_string();
    db.execute_query(
        sqlx::query("INSERT INTO users (id, email, name, password_hash) VALUES ($1, $2, $3, $4)")
            .bind(&id)
            .bind(email)
            .bind(name)
            .bind(&hash),
    )
    .await
    .map_err(|e| Error::BadRequest(format!("register failed: {e}")))?;
    Ok(id)
}

/// Verify credentials, returning the user id.
pub async fn login(db: &DbPool, email: &str, password: &str) -> Result<String> {
    use sqlx::Row;
    let row = db
        .fetch_one(sqlx::query("SELECT id, password_hash FROM users WHERE email = $1").bind(email))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::Unauthorized("unknown email".to_string()))?;
    let id: String = row.try_get("id").map_err(|e| Error::InternalServerError(e.to_string()))?;
    let hash: String = row
        .try_get("password_hash")
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    if !verify_password(password, &hash).map_err(|e| Error::InternalServerError(e.to_string()))? {
        return Err(Error::Unauthorized("wrong password".to_string()));
    }
    Ok(id)
}

/// Mint a JWT for a user id.
pub fn mint(jwt: &JwtManager, user_id: &str) -> Result<String> {
    jwt.generate_token(&Claims::new(user_id.to_string(), 86400))
        .map_err(|e| Error::InternalServerError(e.to_string()))
}

/// Resolve a bearer token to a user id.
pub fn authorize(jwt: &JwtManager, token: &str) -> Result<String> {
    jwt.verify(token)
        .map(|c| c.sub)
        .map_err(|_| Error::Unauthorized("invalid token".to_string()))
}
