//! Todo REST endpoints. The same items are visible through GraphQL.
//!
//! Routes: GET/POST /todos, PUT/DELETE /todos/:id.

use toxi::db::{sqlx, Database};
use toxi::json_response;
use toxi_core::extract::PathParams;
use toxi_core::request::RequestExt;
use toxi_core::{Error, FromRequest, Request, Response, Result, State};
use std::sync::Arc;

use crate::models::{CreateTodo, Todo};
use crate::AppState;

fn id_of(req: &Request) -> String {
    req.extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn row_todo(row: &sqlx::any::AnyRow) -> Result<Todo> {
    use sqlx::Row;
    Ok(Todo {
        id: row.try_get("id").map_err(|e| Error::InternalServerError(e.to_string()))?,
        text: row.try_get("text").map_err(|e| Error::InternalServerError(e.to_string()))?,
        done: row.try_get::<i64, _>("done").map_err(|e| Error::InternalServerError(e.to_string()))? != 0,
    })
}

/// GET /todos — all items.
pub async fn list(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let rows = state
        .db
        .fetch_all(sqlx::query("SELECT id, text, done FROM todos ORDER BY rowid DESC"))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let mut todos = Vec::with_capacity(rows.len());
    for row in &rows {
        todos.push(row_todo(row)?);
    }
    Ok(json_response!({ "todos": todos }))
}

/// POST /todos — create an item.
pub async fn create(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: CreateTodo = req.json().await?;
    if body.text.trim().is_empty() {
        return Err(Error::BadRequest("text is empty".to_string()));
    }
    let todo = Todo {
        id: uuid::Uuid::new_v4().to_string(),
        text: body.text.trim().to_string(),
        done: false,
    };
    state
        .db
        .execute_query(
            sqlx::query("INSERT INTO todos (id, text, done) VALUES ($1, $2, 0)")
                .bind(&todo.id)
                .bind(&todo.text),
        )
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "todo": todo }))
}

/// PUT /todos/:id — flip the done flag.
pub async fn toggle(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let id = id_of(&req);
    state
        .db
        .execute_query(sqlx::query("UPDATE todos SET done = 1 - done WHERE id = $1").bind(&id))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "id": id }))
}

/// DELETE /todos/:id — remove an item.
pub async fn delete(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let id = id_of(&req);
    state
        .db
        .execute_query(sqlx::query("DELETE FROM todos WHERE id = $1").bind(&id))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "deleted": id }))
}
