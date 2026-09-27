use toxi_core::ToxiResponse;


const TOXI_VERSION: &str = env!("CARGO_PKG_VERSION", "0.1.0");

/// Precomputed header values. Formatting and parsing the version string
/// on every response wastes a format plus two parses per request for
/// values that never change within a build.
static SERVER_VALUE: std::sync::OnceLock<http::HeaderValue> = std::sync::OnceLock::new();

fn server_value() -> &'static http::HeaderValue {
    SERVER_VALUE.get_or_init(|| {
        http::HeaderValue::from_str(&format!("Toxi/{}", TOXI_VERSION))
            .unwrap_or_else(|_| http::HeaderValue::from_static("Toxi"))
    })
}

/// Middleware to add Server identification header
pub async fn server_header_middleware(
    mut response: ToxiResponse,
) -> ToxiResponse
{
    // Add Server header
    response.headers_mut().insert(
        http::header::SERVER,
        server_value().clone(),
    );

    // Add X-Powered-By header
    response.headers_mut().insert(
        "x-powered-by",
        http::HeaderValue::from_static("Toxi Framework"),
    );

    response
}

/// Add server headers to response
pub fn add_server_header(mut response: ToxiResponse) -> ToxiResponse {
    response.headers_mut().insert(
        http::header::SERVER,
        server_value().clone(),
    );

    response.headers_mut().insert(
        "x-powered-by",
        http::HeaderValue::from_static("Toxi Framework"),
    );

    response
}
