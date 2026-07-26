# Advanced Features

This document covers the advanced features of the Toxi framework that go beyond basic routing and request handling.

## API Versioning

Toxi supports multiple approaches to API versioning to maintain backward compatibility while evolving your API.

### URL-based Versioning

```rust
use toxi::prelude::*;

let mut router = Router::new();

// Version 1 API
router.get("/api/v1/users", list_users_v1);
router.post("/api/v1/users", create_user_v1);

// Version 2 API
router.get("/api/v2/users", list_users_v2);
router.post("/api/v2/users", create_user_v2);
```

### Header-based Versioning

```rust
use toxi::prelude::*;

async fn versioned_handler(req: Request) -> Result<Response> {
    let version = req.headers()
        .get("accept")
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| {
            if s.contains("version=2") {
                Some(2)
            } else {
                Some(1) // Default version
            }
        })
        .unwrap_or(1);
    
    match version {
        1 => handle_v1_api(req).await,
        2 => handle_v2_api(req).await,
        _ => Err(Error::BadRequest("Unsupported API version".to_string())),
    }
}
```

### Query Parameter Versioning

```rust
use toxi::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiVersionQuery {
    version: Option<u32>,
}

async fn query_versioned_handler(
    req: Request,
    Query(params): Query<ApiVersionQuery>
) -> Result<Response> {
    let version = params.version.unwrap_or(1);
    
    match version {
        1 => handle_v1_api(req).await,
        2 => handle_v2_api(req).await,
        _ => Err(Error::BadRequest("Unsupported API version".to_string())),
    }
}
```

## Background Jobs

Toxi includes a robust background job system for processing tasks asynchronously.

### Job Definition

```rust
use toxi::queue::{Job, Queue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EmailJob {
    recipient: String,
    subject: String,
    body: String,
}

impl Job for EmailJob {
    async fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // Send email logic here
        send_email(&self.recipient, &self.subject, &self.body).await?;
        println!("Email sent to: {}", self.recipient);
        Ok(())
    }
}
```

### Queue Usage

```rust
use toxi::queue::{Queue, Job};

async fn example_job_queue() -> Result<(), Box<dyn std::error::Error>> {
    // Create a queue
    let queue = Queue::memory(); // or Queue::new(Arc::new(RedisBackend::new("redis://...").await?));
    
    // Create a job
    let email_job = EmailJob {
        recipient: "user@example.com".to_string(),
        subject: "Welcome!".to_string(),
        body: "Thank you for joining us.".to_string(),
    };
    
    // Enqueue the job
    queue.enqueue(email_job).await?;
    
    // Process jobs
    queue.process().await?;
    
    Ok(())
}
```

### Scheduled Jobs (Cron)

```rust
use toxi::queue::{Job, Queue};
use cron::Schedule;
use chrono::Utc;

#[derive(Serialize, Deserialize)]
struct DailyReportJob;

impl Job for DailyReportJob {
    async fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // Generate daily report
        generate_daily_report().await?;
        Ok(())
    }
}

async fn schedule_cron_jobs() -> Result<(), Box<dyn std::error::Error>> {
    let queue = Queue::memory();
    
    // Schedule a job to run daily at midnight
    let schedule = Schedule::from_str("0 0 * * * *")?; // Every day at 00:00
    
    // Add to scheduler (conceptual - actual implementation may vary)
    // scheduler.schedule(schedule, DailyReportJob).await?;
    
    Ok(())
}
```

## Real-time Features

Toxi provides real-time communication capabilities through WebSockets and Server-Sent Events (SSE).

### WebSocket Support

```rust
use toxi::prelude::*;
use toxi::realtime::{WebSocketManager, WebSocketConnection};

async fn websocket_handler(
    mut req: Request,
    ws_manager: &WebSocketManager
) -> Result<Response> {
    // Upgrade to WebSocket connection
    if let Some(upgrade) = req.extensions().get::<hyper::upgrade::OnUpgrade>() {
        tokio::spawn(async move {
            if let Ok(upgraded) = upgrade.await {
                let mut ws_conn = WebSocketConnection::new(upgraded);
                
                // Handle WebSocket messages
                while let Some(msg) = ws_conn.recv().await {
                    match msg {
                        Ok(text) => {
                            // Echo the message back
                            ws_conn.send(&format!("Echo: {}", text)).await.unwrap();
                            
                            // Broadcast to all connected clients
                            ws_manager.broadcast(&text).await;
                        }
                        Err(e) => {
                            println!("WebSocket error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
        
        Ok(Response::text("WebSocket upgrade initiated"))
    } else {
        Err(Error::BadRequest("Expected WebSocket upgrade".to_string()))
    }
}
```

### Server-Sent Events (SSE)

```rust
use toxi::prelude::*;
use futures::stream::StreamExt;

async fn sse_handler(_req: Request) -> Result<Response> {
    use futures::stream::iter;
    use tokio_stream::Stream;
    
    let stream = tokio_stream::iter(vec![
        Ok::<_, hyper::Error>(format!("data: {}\n\n", "Connected")),
        Ok::<_, hyper::Error>(format!("data: {}\n\n", "Message 1")),
        Ok::<_, hyper::Error>(format!("data: {}\n\n", "Message 2")),
    ]);
    
    let body = http_body_util::BodyStream::new(stream.map(|item| {
        Ok::<_, hyper::Error>(hyper::body::Frame::data(item.unwrap().into_bytes()))
    }));
    
    let mut response = hyper::Response::builder()
        .header("content-type", "text/event-stream")
        .header("cache-control", "no-cache")
        .header("connection", "keep-alive")
        .body(body)
        .unwrap();
    
    Ok(response)
}
```

## Caching

Toxi provides caching capabilities to improve performance.

### Cache Setup

```rust
use toxi::cache::{Cache, InMemoryCache, RedisCache};

// In-memory cache (for development/simple use)
let memory_cache = InMemoryCache::new();

// Redis cache (for production/distributed systems)
// let redis_cache = RedisCache::new("redis://127.0.0.1:6379").await?;
```

### Cache Operations

```rust
use toxi::cache::Cache;

async fn cache_examples(cache: &impl Cache) -> Result<(), Box<dyn std::error::Error>> {
    // Store data in cache
    cache.set("user:123", "John Doe", Some(std::time::Duration::from_secs(3600))).await?;
    
    // Retrieve data from cache
    if let Some(value) = cache.get::<String>("user:123").await? {
        println!("Cached value: {}", value);
    } else {
        println!("Value not in cache, fetching from database...");
        // Fetch from database and cache it
    }
    
    // Delete from cache
    cache.delete("user:123").await?;
    
    // Clear all cache
    cache.clear().await?;
    
    Ok(())
}
```

## File Storage

Toxi provides file storage capabilities with support for local and cloud storage.

### Local Storage

```rust
use toxi::storage::{Storage, LocalStorage};

let local_storage = LocalStorage::new("./uploads")?;
```

### S3 Storage

```rust
use toxi::storage::{Storage, S3Storage};

let s3_storage = S3Storage::new(
    "access-key".to_string(),
    "secret-key".to_string(),
    "bucket-name".to_string(),
    "region".to_string()
)?;
```

### File Operations

```rust
use toxi::storage::Storage;

async fn file_operations(storage: &impl Storage) -> Result<(), Box<dyn std::error::Error>> {
    // Upload a file
    let file_path = "./path/to/local/file.txt";
    let remote_path = "uploads/file.txt";
    
    storage.upload(file_path, remote_path).await?;
    
    // Download a file
    let downloaded_path = "./downloads/downloaded_file.txt";
    storage.download(remote_path, downloaded_path).await?;
    
    // Get file URL (for public access)
    let file_url = storage.url(remote_path)?;
    println!("File URL: {}", file_url);
    
    // Delete a file
    storage.delete(remote_path).await?;
    
    Ok(())
}
```

## Email Sending

Toxi provides email sending capabilities.

### Email Configuration

```rust
use toxi::mail::{Mailer, SmtpTransport};

let smtp_transport = SmtpTransport::new(
    "smtp.example.com".to_string(),
    587,
    "username".to_string(),
    "password".to_string()
)?;

let mailer = Mailer::new(smtp_transport);
```

### Sending Emails

```rust
use toxi::mail::{Mailer, Message};

async fn send_emails(mailer: &Mailer) -> Result<(), Box<dyn std::error::Error>> {
    let message = Message::new()
        .to("recipient@example.com")
        .subject("Hello from Toxi!")
        .body("This is a test email from the Toxi framework.")
        .html_body("<h1>Hello!</h1><p>This is a <strong>HTML</strong> email.</p>");
    
    mailer.send(message).await?;
    
    Ok(())
}
```

## Security Features

### Rate Limiting

```rust
use toxi_middleware::{RateLimiter, RateLimitConfig};

let rate_limiter = RateLimiter::new(
    RateLimitConfig::new()
        .with_requests_per_minute(100)
        .with_burst_size(10)
);
```

### CSRF Protection

```rust
use toxi_middleware::{CsrfLayer, CsrfConfig};

let csrf_layer = CsrfLayer::new(
    CsrfConfig::new()
        .with_header_name("X-CSRF-Token")
        .with_cookie_name("csrf-token")
);
```

### XSS Sanitization

```rust
use toxi::security::sanitize;

// Sanitize user input to prevent XSS
let user_input = "<script>alert('XSS')</script>";
let sanitized = sanitize::html(user_input);
println!("Sanitized: {}", sanitized); // Safe output
```

## Configuration Management

Toxi provides flexible configuration management.

### Configuration Struct

```rust
use toxi::config::Config;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    auth: AuthConfig,
}

#[derive(Deserialize, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
    ssl_enabled: bool,
}

#[derive(Deserialize, Clone)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
}

#[derive(Deserialize, Clone)]
struct AuthConfig {
    jwt_secret: String,
    session_timeout: u64,
}

// Load configuration
let config: AppConfig = Config::load_from_file("config/app.toml")?;
```

### Environment-based Configuration

```rust
use std::env;

// Load from environment variables
let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
let port = env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .unwrap_or(3000);
```

## Testing Utilities

Toxi provides testing utilities to make testing easier.

### Test Server

```rust
use toxi_testing::{TestServer, TestResponse};

async fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test server with your router
    let server = TestServer::new(|| {
        let mut router = Router::new();
        router.get("/test", |_| async { Ok(Response::text("OK")) });
        router
    })?;
    
    // Make requests to the test server
    let response = server.get("/test").send().await?;
    
    // Assert response
    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await?, "OK");
    
    Ok(())
}
```

### Request Testing

```rust
use toxi_testing::TestRequest;

// Build test requests
let request = TestRequest::get("/")
    .header("authorization", "Bearer token")
    .json(&json!({"key": "value"}));

let response = server.execute(request).await?;
```

## Complete Advanced Example

Here's a complete example demonstrating several advanced features:

```rust
use toxi::prelude::*;
use toxi::auth::{JwtManager, create_token, Claims};
use toxi::db::{Model, Database, DbPool};
use toxi::template::{TemplateEngine, Context};
use toxi_middleware::{ServiceBuilder, LoggerLayer, CorsLayer};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Model, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone)]
struct AppState {
    db: DbPool,
    jwt_manager: JwtManager,
    template_engine: Arc<TemplateEngine>,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>
) -> Result<Response> {
    let mut user = User {
        id: 0,
        name: payload.name,
        email: payload.email,
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };
    
    user.create(&state.db).await
        .map_err(|e| Error::Server(e.to_string()))?;
    
    // Create JWT for the new user
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as u64,
        ..Default::default()
    };
    
    let token = create_token(&state.jwt_manager, claims)
        .map_err(|e| Error::Server(e.to_string()))?;
    
    Ok(json_response!({
        "user": user,
        "token": token
    }))
}

async fn analytics(
    State(state): State<Arc<AppState>>
) -> Result<Response> {
    let users = User::all(&state.db).await
        .map_err(|e| Error::Server(e.to_string()))?;
    
    let mut context = Context::new();
    context.set("users", users);
    context.set("title", "Analytics Dashboard");
    
    let html = state.template_engine.render("analytics.html", &context)
        .map_err(|e| Error::Server(e.to_string()))?;
    
    Ok(Response::html(html))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize components
    let db = DbPool::connect("sqlite::memory:").await?;
    
    // Initialize JWT manager
    let jwt_manager = JwtManager::new("your-secret-key-change-in-production".to_string());
    
    // Initialize template engine
    let mut template_engine = TemplateEngine::new();
    template_engine.add_template("analytics", r#"
    <!DOCTYPE html>
    <html>
    <head><title>{{ title }}</title></head>
    <body>
        <h1>{{ title }}</h1>
        <div>Total Users: {{ users | length }}</div>
        <ul>
        {% for user in users %}
            <li>{{ user.name }} - {{ user.email }}</li>
        {% endfor %}
        </ul>
    </body>
    </html>
    "#)?;
    // Note: For production apps, prefer TemplateContext::new("templates")?.load_dir()?
    
    let app_state = Arc::new(AppState {
        db,
        jwt_manager,
        template_engine: Arc::new(template_engine),
    });
    
    // Setup routes
    let mut router = Router::new();
    router.post("/api/users", create_user);
    router.get("/analytics", analytics);
    
    // Inject state into router
    let router = router.with_state(app_state);
    
    // Apply middleware
    let service = ServiceBuilder::new()
        .layer(LoggerLayer)
        .layer(CorsLayer::permissive())
        .service(router);
    
    println!("Advanced Toxi app running on http://127.0.0.1:3000");
    println!("Features: Database ORM, JWT Authentication, Templates, Middleware");
    println!("API: POST /api/users");
    println!("Analytics: GET /analytics");
    
    Server::new(service)
        .listen("127.0.0.1:3000".parse().unwrap())
        .await
}
```

## Performance Optimization Tips

1. **Connection Pooling**: Always use database connection pools in production
2. **Caching**: Implement strategic caching for expensive operations
3. **Compression**: Enable response compression for larger payloads
4. **Async Operations**: Keep I/O operations asynchronous to maintain throughput
5. **Resource Cleanup**: Properly clean up resources to prevent memory leaks
6. **Monitoring**: Implement metrics and logging for performance monitoring
7. **Database Indexing**: Properly index database tables for optimal queries
8. **Batch Operations**: Use batch operations when processing multiple items