//! Public web page.

use toxi::prelude::*;
use toxi_template::Context;
use std::sync::Arc;

use crate::AppState;

/// GET / — rendered home page with a visit counter.
pub async fn home(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let mut ctx = Context::new();
    ctx.set("title", "Taskboard");
    ctx.set("welcome_message", "Tasks, auth, realtime, uploads");
    let html = state
        .templates
        .render("home.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}
