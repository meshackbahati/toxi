//! Webhook handler: echoes the JSON body with a timestamp.

use toxi::prelude::*;
use toxi::json_response;
use toxi_core::request::RequestExt;

/// POST /hook — echo the JSON body back with a timestamp.
pub async fn hook(mut req: Request) -> Result<Response> {
    let body: serde_json::Value = req.json().await?;
    Ok(json_response!({
        "echo": body,
        "at": chrono::Utc::now().to_rfc3339(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn echo_roundtrip() {
        let req = http::Request::builder()
            .method("POST")
            .uri("/hook")
            .header("content-type", "application/json")
            .body(
                http_body_util::Full::new(bytes::Bytes::from(br#"{"a":1}"#.to_vec()))
                    .map_err(|e| match e {})
                    .boxed(),
            )
            .unwrap();
        let res = hook(req).await.unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
