//! Task CRUD scoped to the bearer-token owner.
//!
//! Routes: GET/POST /tasks, GET/PUT/DELETE /tasks/:id.

use toxi::db::{sqlx, Database};
use toxi::json_response;
use toxi_core::{Error, FromRequest, Request, Response, Result, State, extract::PathParams};
use toxi_core::request::RequestExt;
use std::sync::Arc;

use crate::models::{CreateTask, Task, UpdateTask};
use crate::routes::auth::authenticate;
use crate::AppState;

fn row_task(row: &sqlx::any::AnyRow) -> Result<Task> {
    use sqlx::Row;
    Ok(Task {
        id: row.try_get("id").map_err(|e| Error::InternalServerError(e.to_string()))?,
        user_id: row.try_get("user_id").map_err(|e| Error::InternalServerError(e.to_string()))?,
        title: row.try_get("title").map_err(|e| Error::InternalServerError(e.to_string()))?,
        done: row.try_get::<i64, _>("done").map_err(|e| Error::InternalServerError(e.to_string()))? != 0,
        created_at: row.try_get("created_at").ok(),
    })
}

/// List my tasks, newest first.
pub async fn list(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let claims = authenticate(&mut req, &state).await?;
    let rows = state
        .db
        .fetch_all(sqlx::query("SELECT id, user_id, title, done, created_at FROM tasks WHERE user_id = $1 ORDER BY created_at DESC").bind(&claims.sub))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let mut tasks = Vec::with_capacity(rows.len());
    for row in &rows {
        tasks.push(row_task(row)?);
    }
    Ok(json_response!({ "tasks": tasks }))
}

/// Create a task for me.
pub async fn create(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let claims = authenticate(&mut req, &state).await?;
    let body: CreateTask = req.json().await?;
    if body.title.trim().is_empty() {
        return Err(Error::BadRequest("title is empty".to_string()));
    }
    let task = Task::new(claims.sub.clone(), body.title.trim().to_string());
    state
        .db
        .execute_query(
            sqlx::query("INSERT INTO tasks (id, user_id, title, done) VALUES ($1, $2, $3, 0)")
                .bind(&task.id)
                .bind(&task.user_id)
                .bind(&task.title),
        )
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let _ = state
        .bus
        .publish_message("tasks", serde_json::json!({ "event": "created", "id": task.id }))
        .await;
    Ok(json_response!({ "task": task }))
}

/// Fetch one of my tasks by id.
pub async fn get(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let claims = authenticate(&mut req, &state).await?;
    let id = req
        .extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let row = state
        .db
        .fetch_one(sqlx::query("SELECT id, user_id, title, done, created_at FROM tasks WHERE id = $1 AND user_id = $2").bind(&id).bind(&claims.sub))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    let row = row.ok_or_else(|| Error::NotFound("task not found".to_string()))?;
    Ok(json_response!({ "task": row_task(&row)? }))
}

/// Update title and/or done flag.
pub async fn update(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let claims = authenticate(&mut req, &state).await?;
    let id = req
        .extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let body: UpdateTask = req.json().await?;
    if let Some(title) = &body.title {
        state
            .db
            .execute_query(sqlx::query("UPDATE tasks SET title = $1 WHERE id = $2 AND user_id = $3").bind(title).bind(&id).bind(&claims.sub))
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;
    }
    if let Some(done) = body.done {
        state
            .db
            .execute_query(sqlx::query("UPDATE tasks SET done = $1 WHERE id = $2 AND user_id = $3").bind(if done { 1i64 } else { 0i64 }).bind(&id).bind(&claims.sub))
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;
    }
    Ok(json_response!({ "id": id }))
}

/// Delete one of my tasks.
pub async fn delete(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let claims = authenticate(&mut req, &state).await?;
    let id = req
        .extensions()
        .get::<PathParams>()
        .and_then(|p| p.0.get("id"))
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    state
        .db
        .execute_query(sqlx::query("DELETE FROM tasks WHERE id = $1 AND user_id = $2").bind(&id).bind(&claims.sub))
        .await
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "deleted": id }))
}
