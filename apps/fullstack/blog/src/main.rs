//! Blog: rendered posts with auth-gated writing.
//!
//! Fullstack group: every page is server-rendered HTML. See GUIDE.md.

use toxi::auth::JwtManager;
use toxi::db::DbPool;
use toxi::prelude::*;
use std::sync::Arc;

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub templates: Arc<toxi_template::TemplateContext>,
    pub db: DbPool,
    pub jwt: Arc<JwtManager>,
}

impl AppState {
    async fn load() -> Result<Self> {
        let templates = toxi_template::TemplateContext::new("templates");
        let db_url =
            std::env::var("BLOG_DB").unwrap_or_else(|_| "sqlite:blog.db".to_string());
        let db = DbPool::connect(&db_url)
            .await
            .map_err(|e| Error::InternalServerError(format!("db connect: {e}")))?;
        let sql = std::fs::read_to_string("migrations/001_blog.sql")
            .map_err(|e| Error::InternalServerError(format!("read migration: {e}")))?;
        for statement in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            db.execute(statement)
                .await
                .map_err(|e| Error::InternalServerError(format!("migrate: {e}")))?;
        }
        let secret =
            std::env::var("BLOG_JWT").unwrap_or_else(|_| "blog-dev-secret".to_string());
        Ok(Self {
            templates: Arc::new(templates),
            db,
            jwt: Arc::new(JwtManager::new(secret)),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState::load().await?);
    let mut router = Router::new();

    router.get("/", routes::pages::index);
    router.get("/posts/:id", routes::pages::show);
    router.post("/auth/register", routes::write::register);
    router.post("/auth/login", routes::write::login);
    router.post("/posts", routes::write::publish);
    router.get("/api/status", routes::status::api_status);
    router.get("/health", routes::status::health_check);

    let mut router = router;
    router.with_state(state);
    println!("Blog on http://127.0.0.1:3003");
    Server::new(router).listen("127.0.0.1:3003".parse().unwrap()).await
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
        let dir = std::env::temp_dir().join(format!("blog-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("migrations")).unwrap();
        std::fs::create_dir_all(dir.join("templates")).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        std::fs::copy(
            format!("{}/migrations/001_blog.sql", env!("CARGO_MANIFEST_DIR")),
            "migrations/001_blog.sql",
        )
        .unwrap();
        std::fs::write("templates/index.html", "{% for post in posts %}{{ post.title }}{% endfor %}").unwrap();
        std::fs::write("templates/post.html", "{{ post.title }}").unwrap();
        std::env::set_var("BLOG_DB", "sqlite:blog-test.db");
        Arc::new(AppState::load().await.unwrap())
    }

    #[tokio::test]
    async fn full_flow() {
        let state = test_state().await;

        let res = routes::write::register(
            json_req(
                "POST",
                "/auth/register",
                br#"{"email":"b@b.c","password":"password1","name":"B"}"#,
                &state,
            ),
        )
        .await
        .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let res = routes::pages::index(json_req("GET", "/", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
