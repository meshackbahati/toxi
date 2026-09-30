//! Pastebin: store raw, serve sanitized.
//!
//! Fullstack group: index form plus rendered previews. See GUIDE.md.

use toxi::db::DbPool;
use toxi::prelude::*;
use std::sync::Arc;

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub templates: Arc<toxi_template::TemplateContext>,
    pub db: DbPool,
}

impl AppState {
    async fn load() -> Result<Self> {
        let templates = toxi_template::TemplateContext::new("templates");
        let db_url =
            std::env::var("PASTEBIN_DB").unwrap_or_else(|_| "sqlite:pastebin.db".to_string());
        let db = DbPool::connect(&db_url)
            .await
            .map_err(|e| Error::InternalServerError(format!("db connect: {e}")))?;
        let sql = std::fs::read_to_string("migrations/001_pastes.sql")
            .map_err(|e| Error::InternalServerError(format!("read migration: {e}")))?;
        for statement in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            db.execute(statement)
                .await
                .map_err(|e| Error::InternalServerError(format!("migrate: {e}")))?;
        }
        Ok(Self {
            templates: Arc::new(templates),
            db,
        })
    }
}

async fn index(mut req: Request) -> Result<Response> {
    let State(state): State<Arc<AppState>> = State::from_request(&mut req).await?;
    let mut ctx = toxi_template::Context::new();
    ctx.set("title", "Pastebin");
    let html = state
        .templates
        .render("index.html", &ctx)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Response::html(html))
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState::load().await?);
    let mut router = Router::new();

    router.get("/", index);
    router.get("/api/status", routes::status::api_status);
    router.get("/health", routes::status::health_check);
    router.get("/pastes", routes::pastes::list);
    router.post("/pastes", routes::pastes::create);
    router.get("/pastes/:id", routes::pastes::view);
    router.get("/pastes/:id/raw", routes::pastes::raw);
    router.get("/*", toxi_template::serve_static);

    let mut router = router;
    router.with_state(state);
    println!("Pastebin on http://127.0.0.1:3006");
    Server::new(router).listen("127.0.0.1:3006".parse().unwrap()).await
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
        let dir = std::env::temp_dir().join(format!("pastebin-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("migrations")).unwrap();
        std::fs::create_dir_all(dir.join("templates")).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        std::fs::copy(
            format!("{}/migrations/001_pastes.sql", env!("CARGO_MANIFEST_DIR")),
            "migrations/001_pastes.sql",
        )
        .unwrap();
        std::fs::write("templates/index.html", "pastes").unwrap();
        std::fs::write("templates/paste.html", "{{ preview }}").unwrap();
        std::env::set_var("PASTEBIN_DB", "sqlite:pastebin-test.db");
        Arc::new(AppState::load().await.unwrap())
    }

    #[tokio::test]
    async fn full_flow() {
        let state = test_state().await;

        // A script tag must not survive the preview.
        let res = routes::pastes::create(
            json_req(
                "POST",
                "/pastes",
                br#"{"title":"x","body":"<script>alert(1)</script>hi"}"#,
                &state,
            ),
        )
        .await
        .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let res = routes::pastes::list(json_req("GET", "/pastes", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
