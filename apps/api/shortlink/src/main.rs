use toxi::cache::MemoryCache;
use toxi::db::DbPool;
use toxi::middleware::{RateLimitConfig, RateLimiter};
use toxi::prelude::*;
use std::sync::Arc;

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub cache: Arc<MemoryCache>,
    pub limiter: Arc<RateLimiter>,
}

impl AppState {
    async fn load() -> Result<Self> {
        let db_url =
            std::env::var("SHORTLINK_DB").unwrap_or_else(|_| "sqlite:shortlink.db".to_string());
        let db = DbPool::connect(&db_url)
            .await
            .map_err(|e| Error::InternalServerError(format!("db connect: {e}")))?;
        let sql = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/migrations/001_links.sql"))
            .map_err(|e| Error::InternalServerError(format!("read migration: {e}")))?;
        for statement in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            db.execute(statement)
                .await
                .map_err(|e| Error::InternalServerError(format!("migrate: {e}")))?;
        }
        Ok(Self {
            db,
            cache: Arc::new(MemoryCache::new()),
            limiter: Arc::new(RateLimiter::new(RateLimitConfig {
                requests_per_minute: 60,
                requests_per_hour: Some(1000),
            })),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState::load().await?);
    let mut router = Router::new();

    router.get("/health", routes::status::health_check);
    router.get("/api/status", routes::status::api_status);
    router.post("/links", routes::links::create);
    router.get("/links", routes::links::list);
    router.get("/stats", routes::links::stats);
    router.get("/:code", routes::links::redirect);

    let mut router = router;
    router.with_state(state);
    println!("Shortlink on http://127.0.0.1:3001");
    Server::new(router).listen("127.0.0.1:3001".parse().unwrap()).await
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

    async fn test_state() -> Arc<AppState> {
        let dir = std::env::temp_dir().join(format!("shortlink-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("migrations")).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        std::fs::copy(
            format!("{}/migrations/001_links.sql", env!("CARGO_MANIFEST_DIR")),
            "migrations/001_links.sql",
        )
        .unwrap();
        std::env::set_var("SHORTLINK_DB", "sqlite:shortlink-test.db");
        Arc::new(AppState::load().await.unwrap())
    }

    #[tokio::test]
    async fn full_flow() {
        let state = test_state().await;

        let res = routes::links::create(
            json_req("POST", "/links", br#"{"url":"https://example.com"}"#, &state),
        )
        .await
        .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let res = routes::links::create(
            json_req("POST", "/links", br#"{"url":"not-a-url"}"#, &state),
        )
        .await;
        assert!(res.is_err());

        let res = routes::links::stats(json_req("GET", "/stats", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
