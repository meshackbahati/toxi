//! # Toxi Middleware
//!
//! Standard middleware suite for Toxi applications, built on Tower.
//!
//! ## Available Middleware
//!
//! | Middleware           | Module              | Description                                |
//! |----------------------|---------------------|--------------------------------------------|
//! | CORS                 | `tower_http::cors`  | Cross-Origin Resource Sharing headers      |
//! | Compression          | `tower_http`        | Gzip/Brotli/Deflate response compression   |
//! | Logger               | [`logger`]          | Request/response logging                   |
//! | Rate Limiter         | [`rate_limit`]      | Token-bucket rate limiting                 |
//! | Timeout              | [`timeout`]         | Request deadline enforcement               |
//! | Security Headers     | [`security_headers`] | X-Frame-Options, CSP, HSTS, etc.          |
//! | CSRF                 | [`csrf`]            | Cross-Site Request Forgery protection      |
//! | Request ID           | [`request_id`]      | Unique request ID generation & propagation |
//! | Server Header        | [`server_header`]   | `Server` response header injection         |
//! | Response Cache       | [`cache`]           | In-memory and Redis response caching       |
//! | Metrics              | [`metrics`]         | Request duration and count metrics         |
//!
//! ## Usage
//!
//! Stack middleware using [`tower::ServiceBuilder`]:
//!
//! ```rust,ignore
//! use toxi::prelude::*;
//!
//! let service = ServiceBuilder::new()
//!     .layer(CorsLayer::permissive())
//!     .layer(CompressionLayer::new())
//!     .layer(LoggerLayer::new())
//!     .service(router);
//! ```

/// Re-export commonly used tower-http middleware.
pub use tower_http::compression::CompressionLayer;
pub use tower_http::cors::{CorsLayer, Any};

/// Request/response logging middleware.
pub mod logger;

/// Request ID generation and propagation middleware.
pub mod request_id;

/// Security headers injection middleware (X-Frame-Options, CSP, HSTS, etc.).
pub mod security_headers;

/// CSRF protection middleware.
pub mod csrf;

/// Token-bucket rate limiting middleware.
pub mod rate_limit;

/// Request timeout middleware.
pub mod timeout;

/// Server identification header middleware (`Server: Toxi/x.x.x`).
pub mod server_header;

/// Response caching middleware (in-memory and Redis backends).
pub mod cache;

/// Metrics collection middleware (request duration, counts).
pub mod metrics;

pub use logger::LoggerLayer;
pub use request_id::{RequestIdLayer, RequestIdMiddleware};
pub use security_headers::{SecurityHeadersLayer, SecurityHeadersConfig, FrameOptions};
pub use csrf::{CsrfLayer, CsrfConfig};
pub use rate_limit::{RateLimiter, RateLimitConfig};
pub use timeout::{TimeoutMiddleware, TimeoutError};
pub use server_header::add_server_header;
pub use cache::{CacheLayer, CacheMiddleware, CacheConfig, CacheLayerBuilder};
pub use metrics::{Metrics, MetricsLayer};

/// Re-export [`ServiceBuilder`] for convenient middleware stacking.
pub use tower::ServiceBuilder;

/// Re-export the `tower` crate for service and layer traits.
pub use tower;

/// Re-export the `tower-http` crate for HTTP-specific middleware.
pub use tower_http;
