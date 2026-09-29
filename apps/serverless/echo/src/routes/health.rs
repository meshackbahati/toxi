//! Warmth probe for the function.

use toxi::prelude::*;

/// GET /health — proves the function is warm.
pub async fn health_check(_req: Request) -> Result<Response> {
    Ok(Response::text("OK"))
}
