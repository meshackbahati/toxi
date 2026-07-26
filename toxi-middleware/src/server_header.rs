use toxi_core::ToxiResponse;


const TOXI_VERSION: &str = env!("CARGO_PKG_VERSION", "0.1.0");

/// Middleware to add Server identification header
pub async fn server_header_middleware(
    mut response: ToxiResponse,
) -> ToxiResponse
{
    // Add Server header
    response.headers_mut().insert(
        "server",
        format!("Toxi/{}", TOXI_VERSION).parse().unwrap()
    );
    
    // Add X-Powered-By header
    response.headers_mut().insert(
        "x-powered-by",
        "Toxi Framework".parse().unwrap()
    );
    
    response
}

/// Add server headers to response
pub fn add_server_header(mut response: ToxiResponse) -> ToxiResponse {
    response.headers_mut().insert(
        "server",
        format!("Toxi/{}", TOXI_VERSION).parse().unwrap()
    );
    
    response.headers_mut().insert(
        "x-powered-by",
        "Toxi Framework".parse().unwrap()
    );
    
    response
}
