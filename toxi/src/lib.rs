//! # Toxi Web Framework
//!
//! **Toxi** is a modular, batteries-included web framework for Rust.
//! Built on `hyper` and `tokio`, it provides a complete stack for building
//! production HTTP services — routing, ORM, authentication, real-time
//! communication, background jobs, caching, storage, templates, email,
//! and OpenAPI generation.
//!
//! ## Quick Start
//!
//! ```toml
//! [dependencies]
//! toxi = { version = "3.1.0", features = ["full"] }
//! tokio = { version = "1", features = ["full"] }
//! serde = { version = "1", features = ["derive"] }
//! ```
//!
//! ```rust,no_run
//! use toxi::prelude::*;
//!
//! async fn hello(_req: Request) -> Result<Response> {
//!     Ok(Response::text("Hello, Toxi!"))
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut app = Application::new(Config::load().unwrap());
//!     app.router_mut().get("/", hello);
//!     app.run().await
//! }
//! ```
//!
//! ## Feature Flags
//!
//! Enable only what you need via Cargo features:
//!
//! | Feature     | Crate               | Description                           |
//! |-------------|---------------------|---------------------------------------|
//! | `database`  | `toxi-db`           | ORM with relationships & migrations   |
//! | `auth`      | `toxi-auth`         | JWT, OAuth2, RBAC, 2FA, API keys      |
//! | `queue`     | `toxi-queue`        | Background job orchestration          |
//! | `cache`     | `toxi-cache`        | In-memory and Redis caching           |
//! | `realtime`  | `toxi-realtime`     | WebSocket and SSE                     |
//! | `templates` | `toxi-template`     | Server-side rendering                 |
//! | `mail`      | `toxi-mail`         | SMTP email delivery                   |
//! | `storage`   | `toxi-storage`      | Local and S3 file storage             |
//! | `security`  | `toxi-security`     | Crypto primitives                     |
//! | `utils`     | `toxi-utils`        | String, date, validation helpers      |
//! | `graphql`   | `toxi-graphql`      | GraphQL integration                   |
//! | `plugin`    | `toxi-plugin`       | Plugin lifecycle hooks                |
//! | `full`      | all of the above    | Every optional crate                  |
//!
//! ## Modular by Design
//!
//! Each crate in the Toxi ecosystem can be used independently:
//!
//! ```toml
//! # Minimal: just routing and HTTP
//! [dependencies]
//! toxi-core = "3.1.0"
//!
//! # Add database when needed
//! [dependencies]
//! toxi-core = "3.1.0"
//! toxi-db = "3.1.0"
//! ```
//!
//! ## Key Concepts
//!
//! - **Extractors** — Typed data pulled from requests (`Json<T>`, `Path<T>`, `State<T>`, etc.)
//! - **Handlers** — Async functions whose parameters are extractors
//! - **Middleware** — Tower-compatible layers (CORS, logging, compression, rate limiting)
//! - **HTTP/2** — First-class support via ALPN or `Server::with_http_version`
//!
//! ## Learn More
//!
//! - [GitHub](https://github.com/meshackbahati/toxi)
//! - [API Documentation](https://docs.rs/toxi)

// ── Core re-exports ──────────────────────────────────────────────────

/// Re-export all items from [`toxi_core`].
pub use toxi_core::*;

/// Re-export the `extract` module from [`toxi_core`].
pub use toxi_core::extract;

/// Re-export [`toxi_middleware`] as `middleware`.
pub use toxi_middleware as middleware;

/// Re-export [`toxi_config`] as `config`.
pub use toxi_config as config;

/// Re-export [`ToxiRequest`] and [`ToxiResponse`].
pub use toxi_core::types::{ToxiRequest, ToxiResponse};

// ── Macro re-exports ─────────────────────────────────────────────────

/// Re-export `impl_handler_for_fn!` — extend handler arity beyond the
/// built-in 12 extractors:
///
/// ```rust,ignore
/// // Enable 13 and 14 extractor handlers
/// toxi::impl_handler_for_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
/// ```
pub use toxi_core::impl_handler_for_fn;

/// Re-export `json_response!` — create a JSON response from inline JSON:
///
/// ```rust,ignore
/// use toxi::prelude::*;
///
/// async fn health() -> Result<Response> {
///     Ok(json_response!({ "status": "ok" }))
/// }
/// ```
pub use toxi_core::json_response;

// ── Optional crate re-exports (behind feature flags) ─────────────────

#[cfg(feature = "database")]
/// Re-export [`toxi_db`] as `db` — ORM with relationships, migrations,
/// soft deletes, and eager loading.
pub use toxi_db as db;

#[cfg(feature = "auth")]
/// Re-export [`toxi_auth`] as `auth` — JWT, OAuth2, RBAC, 2FA, and API keys.
pub use toxi_auth as auth;

#[cfg(feature = "queue")]
/// Re-export [`toxi_queue`] as `queue` — background job orchestration
/// with Postgres and Redis backends.
pub use toxi_queue as queue;

#[cfg(feature = "cache")]
/// Re-export [`toxi_cache`] as `cache` — in-memory and Redis caching.
pub use toxi_cache as cache;

#[cfg(feature = "realtime")]
/// Re-export [`toxi_realtime`] as `realtime` — WebSocket, SSE, and
/// event broadcasting.
pub use toxi_realtime as realtime;

#[cfg(feature = "templates")]
/// Re-export [`toxi_template`] as `template` — server-side rendering.
pub use toxi_template as template;

#[cfg(feature = "mail")]
/// Re-export [`toxi_mail`] as `mail` — SMTP email delivery.
pub use toxi_mail as mail;

#[cfg(feature = "storage")]
/// Re-export [`toxi_storage`] as `storage` — local and S3 file storage.
pub use toxi_storage as storage;

#[cfg(feature = "security")]
/// Re-export [`toxi_security`] as `security` — cryptographic primitives.
pub use toxi_security as security;

#[cfg(feature = "utils")]
/// Re-export [`toxi_utils`] as `utils` — string, date, and validation helpers.
pub use toxi_utils as utils;

// ── Prelude ──────────────────────────────────────────────────────────

/// Convenience re-exports for ergonomic imports.
///
/// ```rust,ignore
/// use toxi::prelude::*;
/// ```
pub mod prelude {
    // Core
    /// Core framework types: `Application`, `Router`, `Server`, `Handler`,
    /// `Error`, `Result`, `Request`, `Response`, extractors, and more.
    pub use toxi_core::{
        Application, Router, Server, Handler, IntoHandler, handler_fn,
        Error, Result,
        Request, Response,
        StatusCode, mpsc, BodyExt,
        CorsConfig, HttpVersion,
        extract::{Json, Path, Query, State, FromRequest, Form, Cookies, Body, WebSocketUpgrade, PathParams},
    };

    // Middleware
    /// Middleware types: `ServiceBuilder`, `LoggerLayer`, `CorsLayer`,
    /// `CompressionLayer`, `CacheLayer`, `MetricsLayer`.
    pub use toxi_middleware::{
        ServiceBuilder, LoggerLayer, CorsLayer, CompressionLayer,
        CacheLayer, CacheMiddleware, CacheConfig, CacheLayerBuilder,
        MetricsLayer,
    };

    // Config
    /// Application configuration loaded from `toxi.toml` or environment variables.
    pub use toxi_config::Config;

    // Response helpers
    /// Response helper functions: `json!({...})`, `text("...")`, `html("...")`,
    /// `ok()`, `no_content()`.
    pub use toxi_core::response::helpers::{json, text, html, ok, no_content};

    // Database
    #[cfg(feature = "database")]
    /// Database types: `Database`, `Model`, `Migration`, `DbPool`, `DbTransaction`,
    /// `OrmError`, `Pagination`, `SortDirection`.
    pub use toxi_db::{Database, Model, Migration, DbPool, DbTransaction, Result as DbResult, OrmError, Pagination, SortDirection};

    #[cfg(feature = "database")]
    /// Commonly used `sqlx` types re-exported for convenience so you don't
    /// need to depend on `sqlx` directly.
    pub use toxi_db::sqlx::{
        query, query_as, query_scalar,
        FromRow, Row,
        Encode, Decode, Type,
        AnyPool, Any,
        postgres::{PgPool, PgRow},
        mysql::{MySqlPool, MySqlRow},
        sqlite::{SqlitePool, SqliteRow},
        Column,
        Acquire, Executor,
    };

    // Auth
    #[cfg(feature = "auth")]
    /// Auth types: `Permission`, `Role`.
    pub use toxi_auth::{Permission, Role};

    // Queue
    #[cfg(feature = "queue")]
    /// Queue types: `Queue`, `Job`, `PostgresBackend`.
    pub use toxi_queue::{Queue, Job, PostgresBackend};

    // Cache
    #[cfg(feature = "cache")]
    /// Cache trait and backends.
    pub use toxi_cache::Cache;

    // Realtime
    #[cfg(feature = "realtime")]
    /// WebSocket manager for real-time connections.
    pub use toxi_realtime::WebSocketManager;

    // GraphQL
    #[cfg(feature = "graphql")]
    /// GraphQL handler and schema types.
    pub use toxi_graphql::{GraphQLHandler, GraphQLSchema};

    // Plugin
    #[cfg(feature = "plugin")]
    /// Plugin manager, trait, and info types.
    pub use toxi_plugin::{PluginManager, Plugin, PluginInfo};

    // Serde
    /// `Serialize` and `Deserialize` derive macros.
    pub use serde::{Serialize, Deserialize};

    /// `serde_json::json!` macro for inline JSON construction.
    pub use serde_json::json;

    /// Build a CORS layer from configuration.
    pub use crate::config_helper::cors_layer_from_config;
}

/// Configuration helper utilities.
pub mod config_helper;

/// Re-export of [`cors_layer_from_config`].
pub use config_helper::cors_layer_from_config;
