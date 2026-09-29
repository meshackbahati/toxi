//! Blog: rendered posts with auth-gated writing.
//!
//! Boot sequence: Config ──> Router ──> Middleware ──> Server.
//! See GUIDE.md.

use toxi::auth::JwtManager;
use toxi::db::DbPool;
use toxi::prelude::*;
use std::sync::Arc;

mod config;
mod controllers;
mod middleware;
mod models;
mod routes;
mod services;
mod validators;

#[derive(Clone)]
pub struct AppState {
    pub templates: Arc<toxi_template::TemplateContext>,
    pub db: DbPool,
    pub jwt: Arc<JwtManager>,
}

impl AppState {
    async fn load(settings: &config::Settings) -> Result<Self> {
        let templates = toxi_template::TemplateContext::new("templates");
        let db = DbPool::connect(&settings.db_url)
            .await
            .map_err(|e| Error::InternalServerError(format!("db connect: {e}")))?;
        let sql = std::fs::read_to_string("migrations/001_blog.sql")
            .map_err(|e| Error::InternalServerError(format!("read migration: {e}")))?;
        for statement in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            db.execute(statement)
                .await
                .map_err(|e| Error::InternalServerError(format!("migrate: {e}")))?;
        }
        let state = Self {
            templates: Arc::new(templates),
            db,
            jwt: Arc::new(JwtManager::new(settings.jwt_secret.clone())),
        };
        state.seed_welcome().await?;
        Ok(state)
    }

    /// First boot plants a welcome post so the index never opens empty.
    async fn seed_welcome(&self) -> Result<()> {
        use toxi::db::Database;
        let existing: Option<i64> = self
            .db
            .fetch_one(toxi::db::sqlx::query("SELECT COUNT(*) AS n FROM posts"))
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?
            .and_then(|row| {
                use toxi::db::sqlx::Row;
                row.try_get("n").ok()
            });
        if existing.unwrap_or(0) > 0 {
            return Ok(());
        }
        self.db
            .execute("INSERT INTO users (id, email, name) VALUES ('system', 'system@local', 'System')")
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;
        self.db
            .execute(
                "INSERT INTO posts (id, user_id, title, body) VALUES \
                 ('welcome', 'system', 'Welcome to the blog', \
                 'This is the first post. Register an account to publish your own.')",
            )
            .await
            .map_err(|e| Error::InternalServerError(e.to_string()))?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // ── 1. Config ──────────────────────────────────────────────────
    let settings = config::load()?;

    // Shared state travels in request extensions for State.
    let state = Arc::new(AppState::load(&settings).await?);

    // ── 2. Router ──────────────────────────────────────────────────
    let mut app = Application::new(
        toxi::config::Config::load()
            .map_err(|e| Error::InternalServerError(format!("config: {e}")))?,
    );
    routes::register(app.router_mut());
    app.router_mut().with_state(state);

    // ── 3. Middleware ── 4. Server ─────────────────────────────────
    println!(
        "Blog on http://{}:{}",
        app.config().server.host,
        app.config().server.port
    );
    let router = app.into_router();
    let logged = tower::ServiceBuilder::new()
        .layer(middleware::LoggerLayer)
        .service(router);
    let addr: std::net::SocketAddr = format!("{}:{}", settings.host, settings.port)
        .parse()
        .unwrap();
    Server::new(logged).listen(addr).await
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
        std::fs::write("templates/index.html", "{% for post in posts %}{{ post.title }}{% endfor %}{% if post_count == 0 %}empty{% endif %}").unwrap();
        std::fs::write("templates/post.html", "{{ post.title }}").unwrap();
        std::env::set_var("BLOG_DB", "sqlite:blog-test.db");
        let settings = config::Settings {
            host: "127.0.0.1".to_string(),
            port: 3003,
            db_url: "sqlite:blog-test.db".to_string(),
            jwt_secret: "test-secret".to_string(),
        };
        Arc::new(AppState::load(&settings).await.unwrap())
    }

    #[tokio::test]
    async fn full_flow() {
        let state = test_state().await;

        let res = controllers::write::register(
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

        let res = controllers::pages::index(json_req("GET", "/", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
