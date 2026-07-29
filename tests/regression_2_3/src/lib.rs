#[tokio::test]
async fn test_12_extractors_compilation() {
    use toxi::prelude::*;

    async fn h12(
        _e1: State<()>, _e2: State<()>, _e3: State<()>, _e4: State<()>,
        _e5: State<()>, _e6: State<()>, _e7: State<()>, _e8: State<()>,
        _e9: State<()>, _e10: State<()>, _e11: State<()>, _e12: State<()>,
    ) -> Result<Response> {
        Ok(Response::text("ok"))
    }

    let mut router = Router::new();
    router.with_state(());
    router.get("/", h12);
}

#[test]
fn test_orm_error_not_found_id_type() {
    let err = toxi::db::OrmError::NotFound {
        model: "User",
        id: "abc-123".to_string(),
    };
    assert_eq!(err.to_string(), "model `User` with id `abc-123` was not found");
}

#[tokio::test]
async fn test_http_version_auto_cleartext() {
    use toxi::prelude::*;

    async fn index(_req: Request) -> Result<Response> {
        Ok(Response::text("auto-detect-ok"))
    }

    let mut router = Router::new();
    router.get("/", index);

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let server = Server::new(router)
        .with_http_version(HttpVersion::Auto)
        .bind(addr);

    // Spawn server
    let server_handle = tokio::spawn(async move {
        server.run().await.unwrap();
    });

    // Give server a tiny bit of time to start up
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;

    // 1. Test HTTP/1.1 request
    let client_http1 = reqwest::Client::new();

    let res_http1 = client_http1
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(res_http1.status(), reqwest::StatusCode::OK);
    assert_eq!(res_http1.version(), reqwest::Version::HTTP_11);
    let body_http1 = res_http1.text().await.unwrap();
    assert_eq!(body_http1, "auto-detect-ok");

    // 2. Test HTTP/2 request (cleartext / h2c)
    let client_http2 = reqwest::Client::builder()
        .http2_prior_knowledge()
        .build()
        .unwrap();

    let res_http2 = client_http2
        .get(format!("http://{}/", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(res_http2.status(), reqwest::StatusCode::OK);
    assert_eq!(res_http2.version(), reqwest::Version::HTTP_2);
    let body_http2 = res_http2.text().await.unwrap();
    assert_eq!(body_http2, "auto-detect-ok");

    // Cleanup spawned task
    server_handle.abort();
}
