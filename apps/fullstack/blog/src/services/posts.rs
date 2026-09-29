//! Post queries. Controllers stay thin; SQL lives here.

use toxi::db::{sqlx, Database, DbPool};
use toxi_core::{Error, Result};

use crate::models::Post;

/// Newest posts first, capped at `limit`.
pub async fn latest(db: &DbPool, limit: i64) -> Result<Vec<Post>> {
    use sqlx::Row;
    let rows = db
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

/// Total post count, for the index empty state.
pub async fn count(db: &DbPool) -> Result<i64> {
    use sqlx::Row;
    let row = db
        .fetch_one(sqlx::query("SELECT COUNT(*) AS n FROM posts"))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(row.and_then(|r| r.try_get("n").ok()).unwrap_or(0))
}

/// One post by id, with its author name.
pub async fn one(db: &DbPool, id: &str) -> Result<Option<Post>> {
    use sqlx::Row;
    let row = db
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

/// Insert a post, returning its id.
pub async fn insert(db: &DbPool, user_id: &str, title: &str, body: &str) -> Result<String> {
    let id = uuid::Uuid::new_v4().to_string();
    db.execute_query(
        sqlx::query("INSERT INTO posts (id, user_id, title, body) VALUES ($1, $2, $3, $4)")
            .bind(&id)
            .bind(user_id)
            .bind(title)
            .bind(body),
    )
    .await
    .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(id)
}
