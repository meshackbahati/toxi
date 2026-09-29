//! Echo: serverless-style webhook on Toxi.
//!
//! One stateless handler plus a warmth probe. See GUIDE.md.

use toxi::prelude::*;

mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = Router::new();
    router.post("/hook", routes::hook::hook);
    router.get("/health", routes::health::health_check);
    println!("Echo on http://127.0.0.1:3002");
    Server::new(router).listen("127.0.0.1:3002".parse().unwrap()).await
}
