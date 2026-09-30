//! Subscription endpoints: enqueue welcomes, run the worker, stats.
//!
//! Routes: POST /subscribe, POST /work, GET /stats, GET /dead-letter.

use toxi::json_response;
use toxi::prelude::*;
use toxi_core::request::RequestExt;
use toxi_queue::job::JobWrapper;
use std::sync::Arc;

use crate::jobs::WelcomeEmail;
use crate::AppState;

#[derive(serde::Deserialize)]
struct Subscribe {
    email: String,
    name: String,
}

/// POST /subscribe — queue a welcome email.
pub async fn subscribe(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let body: Subscribe = req.json().await?;
    if !body.email.contains('@') {
        return Err(Error::BadRequest("invalid email".to_string()));
    }
    let job = WelcomeEmail {
        email: body.email.clone(),
        name: body.name.clone(),
    };
    let wrapper =
        JobWrapper::new(&job).map_err(|e| Error::InternalServerError(e.to_string()))?;
    let id = state.queue.enqueue(wrapper).await.map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "queued": id, "email": body.email }))
}

/// POST /work — run one queued job through SMTP now.
pub async fn work(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let next = state.queue.dequeue().await.map_err(|e| Error::InternalServerError(e.to_string()))?;
    let Some(job) = next else {
        return Ok(json_response!({ "worked": false }));
    };
    let email: WelcomeEmail = serde_json::from_value(job.payload.clone())
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    match toxi_queue::Job::perform(&email).await {
        Ok(()) => {
            state.queue.complete(&job.id).await.map_err(|e| Error::InternalServerError(e.to_string()))?;
            Ok(json_response!({ "worked": true, "id": job.id }))
        }
        Err(e) => {
            state.queue.fail(&job.id, e.to_string()).await.map_err(|e| Error::InternalServerError(e.to_string()))?;
            Ok(json_response!({ "worked": false, "id": job.id, "error": e.to_string() }))
        }
    }
}

/// GET /stats — queue counters.
pub async fn stats(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let s = state.queue.get_stats().await;
    Ok(json_response!({
        "enqueued": s.total_enqueued,
        "processed": s.total_processed,
        "failed": s.total_failed,
        "pending": s.pending_count,
    }))
}

/// GET /dead-letter — permanently failed jobs.
pub async fn dead_letter(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let jobs = state.queue.list_dead_letter().await.map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(json_response!({ "dead_letter": jobs }))
}
