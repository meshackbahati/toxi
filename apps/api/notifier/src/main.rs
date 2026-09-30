//! Notifier: queued welcome emails over SMTP.
//!
//! POST /subscribe queues; POST /work sends one now. Point SMTP_HOST
//! and SMTP_PORT at Mailpit (localhost:1025) to watch delivery. See GUIDE.md.

use toxi::prelude::*;
use toxi_queue::Queue;
use std::sync::Arc;

mod jobs;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub queue: Arc<Queue>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState {
        queue: Arc::new(Queue::memory()),
    });
    let mut router = Router::new();

    router.get("/health", routes::status::health_check);
    router.get("/api/status", routes::status::api_status);
    router.post("/subscribe", routes::subscribe::subscribe);
    router.post("/work", routes::subscribe::work);
    router.get("/stats", routes::subscribe::stats);
    router.get("/dead-letter", routes::subscribe::dead_letter);

    let mut router = router;
    router.with_state(state);
    println!("Notifier on http://127.0.0.1:3005");
    Server::new(router).listen("127.0.0.1:3005".parse().unwrap()).await
}

#[cfg(test)]
mod boot_tests {
    use super::*;
    use http_body_util::BodyExt;

    fn json_req(method: &str, uri: &str, body: &[u8], state: &Arc<AppState>) -> Request {
        let mut req = http::Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(
                http_body_util::Full::new(bytes::Bytes::from(body.to_vec()))
                    .map_err(|e| match e {})
                    .boxed(),
            )
            .unwrap();
        req.extensions_mut().insert(state.clone());
        req
    }

    #[tokio::test]
    async fn full_flow() {
        let state = Arc::new(AppState {
            queue: Arc::new(Queue::memory()),
        });

        let res = routes::subscribe::subscribe(
            json_req("POST", "/subscribe", br#"{"email":"n@n.c","name":"N"}"#, &state),
        )
        .await
        .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let res = routes::subscribe::stats(json_req("GET", "/stats", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
