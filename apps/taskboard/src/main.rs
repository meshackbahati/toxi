use toxi::auth::JwtManager;
use toxi::db::DbPool;
use toxi::prelude::*;
use toxi::realtime::PubSub;
use toxi::json_response;
use toxi_template::{Context, TemplateContext};
use std::sync::Arc;

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub templates: Arc<TemplateContext>,
    pub db: DbPool,
    pub jwt: Arc<JwtManager>,
    pub bus: Arc<PubSub>,
}

impl AppState {
    async fn load() -> Result<Self> {
        // Templates resolve relative to the app directory.
        let templates = TemplateContext::new("templates");
        let db_url =
            std::env::var("TASKBOARD_DB").unwrap_or_else(|_| "sqlite:taskboard.db".to_string());
        let db = DbPool::connect(&db_url)
            .await
            .map_err(|e| Error::InternalServerError(format!("db connect: {e}")))?;
        for file in [
            "migrations/001_initial_schema.sql",
            "migrations/002_tasks.sql",
            "migrations/003_password_hash.sql",
        ] {
            let sql = std::fs::read_to_string(file)
                .map_err(|e| Error::InternalServerError(format!("read {file}: {e}")))?;
            for statement in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
                db.execute(statement)
                    .await
                    .map_err(|e| Error::InternalServerError(format!("migrate: {e}")))?;
            }
        }
        let secret =
            std::env::var("TASKBOARD_JWT").unwrap_or_else(|_| "taskboard-dev-secret".to_string());
        Ok(Self {
            templates: Arc::new(templates),
            db,
            jwt: Arc::new(JwtManager::new(secret)),
            bus: Arc::new(PubSub::new()),
        })
    }
}

async fn home(mut req: Request) -> Result<Response> {
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

async fn api_status(_req: Request) -> Result<Response> {
    Ok(json_response!({
        "status": "online",
        "framework": "Toxi",
        "app": "taskboard",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

async fn health_check(_req: Request) -> Result<Response> {
    Ok(Response::text("OK"))
}

#[derive(serde::Serialize)]
struct UserSummary {
    id: u64,
    name: String,
    active: bool,
}

#[derive(serde::Deserialize, Default)]
struct Pagination {
    page: Option<u64>,
    limit: Option<u64>,
}

async fn get_users(Query(params): Query<Pagination>) -> Result<Response> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    let users: Vec<UserSummary> = ((offset + 1)..=(offset + limit))
        .map(|i| UserSummary {
            id: i,
            name: format!("User {i}"),
            active: true,
        })
        .collect();
    Ok(json_response!({
        "users": users,
        "pagination": { "page": page, "limit": limit, "offset": offset },
    }))
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState::load().await?);
    let mut router = Router::new();

    router.get("/", home);
    router.get("/api/status", api_status);
    router.get("/health", health_check);
    router.get("/users", get_users);

    router.post("/auth/register", routes::auth::register);
    router.post("/auth/login", routes::auth::login);
    router.get("/auth/me", routes::auth::me);

    router.get("/tasks", routes::tasks::list);
    router.post("/tasks", routes::tasks::create);
    router.get("/tasks/:id", routes::tasks::get);
    router.put("/tasks/:id", routes::tasks::update);
    router.delete("/tasks/:id", routes::tasks::delete);

    router.post("/uploads", routes::uploads::upload);
    router.get("/uploads/:name", routes::uploads::download);
    router.get("/events/next", routes::realtime::next);
    router.get("/openapi.json", routes::openapi_spec);
    router.get("/favicon.ico", routes::favicon);

    // Shared state travels in request extensions for State.
    let mut router = router;
    router.with_state(state);
    println!("Taskboard on http://127.0.0.1:3000");
    Server::new(router).listen("127.0.0.1:3000".parse().unwrap()).await
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
        let dir = std::env::temp_dir().join(format!("taskboard-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        std::fs::create_dir_all("templates").unwrap();
        std::fs::write("templates/home.html", "hi").unwrap();
        std::fs::create_dir_all("migrations").unwrap();
        for f in [
            "migrations/001_initial_schema.sql",
            "migrations/002_tasks.sql",
            "migrations/003_password_hash.sql",
        ] {
            std::fs::copy(format!("{}/{f}", env!("CARGO_MANIFEST_DIR")), f).unwrap();
        }
        std::env::set_var("TASKBOARD_DB", "sqlite:taskboard-test.db");
        Arc::new(AppState::load().await.unwrap())
    }

    #[tokio::test]
    async fn full_flow() {
        let state = test_state().await;

        let res = routes::auth::register(
            json_req(
                "POST",
                "/auth/register",
                br#"{"email":"t@t.c","password":"password1","name":"T"}"#,
                &state,
            ),
        )
        .await
        .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let req = json_req(
            "POST",
            "/auth/login",
            br#"{"email":"t@t.c","password":"password1"}"#,
            &state,
        );
        let res = routes::auth::login(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let res = routes::tasks::create(
            json_req("POST", "/tasks", br#"{"title":"x"}"#, &state),
        )
        .await;
        // No token yet: must be unauthorized.
        assert!(res.is_err());

        let res = routes::openapi_spec(json_req("GET", "/openapi.json", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
