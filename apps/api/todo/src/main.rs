//! Todo: REST plus the framework GraphQL schema.
//!
//! The mounted `/graphql` endpoint serves `toxi-graphql` with its
//! built-in schema; the REST routes below persist the same kind of
//! items in sqlite. See GUIDE.md.

use toxi::db::DbPool;
use toxi::prelude::*;
use std::sync::Arc;

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
}

impl AppState {
    async fn load() -> Result<Self> {
        let db_url =
            std::env::var("TODO_DB").unwrap_or_else(|_| "sqlite:todo.db".to_string());
        let db = DbPool::connect(&db_url)
            .await
            .map_err(|e| Error::InternalServerError(format!("db connect: {e}")))?;
        let sql = std::fs::read_to_string("migrations/001_todos.sql")
            .map_err(|e| Error::InternalServerError(format!("read migration: {e}")))?;
        for statement in sql.split(';').map(str::trim).filter(|s| !s.is_empty()) {
            db.execute(statement)
                .await
                .map_err(|e| Error::InternalServerError(format!("migrate: {e}")))?;
        }
        Ok(Self { db })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AppState::load().await?);
    let mut router = Router::new();

    router.get("/health", routes::status::health_check);
    router.get("/api/status", routes::status::api_status);
    router.get("/todos", routes::todos::list);
    router.post("/todos", routes::todos::create);
    router.put("/todos/:id", routes::todos::toggle);
    router.delete("/todos/:id", routes::todos::delete);

    // Framework GraphQL schema with its playground, mounted as-is.
    toxi::prelude::GraphQLHandler::new(toxi_graphql::schema::create_schema())
        .mount(&mut router)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;

    let mut router = router;
    router.with_state(state);
    println!("Todo on http://127.0.0.1:3004");
    Server::new(router).listen("127.0.0.1:3004".parse().unwrap()).await
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
        let dir = std::env::temp_dir().join(format!("todo-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("migrations")).unwrap();
        std::env::set_current_dir(&dir).unwrap();
        std::fs::copy(
            format!("{}/migrations/001_todos.sql", env!("CARGO_MANIFEST_DIR")),
            "migrations/001_todos.sql",
        )
        .unwrap();
        std::env::set_var("TODO_DB", "sqlite:todo-test.db");
        Arc::new(AppState::load().await.unwrap())
    }

    #[tokio::test]
    async fn full_flow() {
        let state = test_state().await;

        let res = routes::todos::create(
            json_req("POST", "/todos", br#"{"text":"buy milk"}"#, &state),
        )
        .await
        .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);

        let res = routes::todos::list(json_req("GET", "/todos", b"", &state))
            .await
            .unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
