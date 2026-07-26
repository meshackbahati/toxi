# Deploying Toxi on AWS

## Elastic Beanstalk

1. Install the EB CLI and initialize:
```bash
eb init --platform "Rust 1.75" --region us-east-1 my-toxi-app
```
2. Configure environment properties via `.ebextensions/toxi.config`:
```yaml
option_settings:
  aws:elasticbeanstalk:application:environment:
    TOXI_ENV: production
    RUST_LOG: info
    PORT: "8080"
    DATABASE_URL: "<rds-endpoint>"
```
3. Deploy:
```bash
eb create toxi-production --instance-type t3.medium --scale 2
```

## ECS / Fargate with Docker

Dockerfile:
```dockerfile
FROM rust:1.75-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/toxi-app /usr/local/bin/
EXPOSE 3000
CMD ["toxi-app"]
```

Task definition snippet:
```json
{
  "family": "toxi-app",
  "networkMode": "awsvpc",
  "containerDefinitions": [{
    "name": "toxi-app",
    "image": "<account>.dkr.ecr.us-east-1.amazonaws.com/toxi-app:latest",
    "portMappings": [{"containerPort": 3000, "protocol": "tcp"}],
    "environment": [
      {"name": "TOXI_ENV", "value": "production"},
      {"name": "RUST_LOG", "value": "info"}
    ],
    "logConfiguration": {
      "logDriver": "awslogs",
      "options": {
        "awslogs-group": "/ecs/toxi-app",
        "awslogs-region": "us-east-1",
        "awslogs-stream-prefix": "toxi"
      }
    }
  }]
}
```

## ALB + EC2 Configuration

Attach an Application Load Balancer in front of an EC2 Auto Scaling group. Enable health checks on `/health`:

```rust
// src/health.rs
use toxi::prelude::*;

pub async fn health_check() -> Result<Response> {
    Ok(Response::json(serde_json::json!({"status": "ok"})))
}
```

Register the target group with stickiness *disabled* — Toxi is stateless.

## WebSocket Support with ALB

ALB supports WebSocket natively (no stickiness required). Ensure `idle_timeout.timeout_seconds = 60` in the target group. Toxi handles upgrade automatically:

```rust
use toxi::realtime::{WebSocketManager, WebSocketConnection};

async fn ws_handler(req: Request, ws_manager: &WebSocketManager) -> Result<Response> {
    if let Some(upgrade) = req.extensions().get::<hyper::upgrade::OnUpgrade>() {
        tokio::spawn(async move {
            if let Ok(upgraded) = upgrade.await {
                let mut conn = WebSocketConnection::new(upgraded);
                while let Some(msg) = conn.recv().await {
                    if let Ok(text) = msg {
                        ws_manager.broadcast(&text).await;
                    }
                }
            }
        });
        Ok(Response::text("connected"))
    } else {
        Err(Error::BadRequest("expected upgrade"))
    }
}
```

## RDS Database Setup

```bash
aws rds create-db-instance \
  --engine postgres \
  --db-instance-class db.t3.medium \
  --db-instance-identifier toxi-db \
  --master-username toxi_admin \
  --master-user-password <secret>
```

Set `DATABASE_URL=postgres://toxi_admin:<secret>@<endpoint>:5432/toxi` in environment.

## Environment Variables

| Variable       | Purpose                         |
|----------------|---------------------------------|
| `TOXI_ENV`     | `production` / `staging`        |
| `RUST_LOG`     | Log level (`info`, `debug`)     |
| `PORT`         | HTTP listen port (default 3000) |
| `DATABASE_URL` | Full database connection string |

## CloudWatch Logging

Configure `RUST_LOG=info` and stream to CloudWatch via the `awslogs` driver (see ECS task above). For EC2, install the CloudWatch agent:

```json
{
  "logs": {
    "logs_collected": {
      "files": {
        "collect_list": [{
          "file_path": "/var/log/toxi-app.log",
          "log_group_name": "/ec2/toxi-app",
          "auto_removal": false
        }]
      }
    }
  }
}
```

## Auto-scaling Considerations

- Set CPU target tracking at 70% average utilization.
- Configure step scaling on ALB `RequestCountPerTarget` > 5000.
- Use a lifecycle hook for graceful shutdown — Toxi catches `SIGTERM` and drains connections.
- Keep Toxi processes stateless (session data in Redis/ElastiCache).

## `toxi.toml` — Production Example

```toml
[server]
host = "0.0.0.0"
port = 3000
workers = 4
max_connections = 10000

[database]
pool_size = 20
max_lifetime_secs = 1800

[logging]
format = "json"
level = "info"

[websocket]
max_message_size = 65536
idle_timeout_secs = 120
```
