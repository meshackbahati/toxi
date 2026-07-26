# Deploying Toxi on GCP

## Cloud Run (Serverless Containers)

1. Build and push the container:
```bash
gcloud builds submit --tag gcr.io/<project>/toxi-app
```
2. Deploy with minimal config:
```bash
gcloud run deploy toxi-app \
  --image gcr.io/<project>/toxi-app \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars "TOXI_ENV=production,RUST_LOG=info" \
  --concurrency 80 \
  --memory 512Mi \
  --cpu 1
```
Cloud Run injects `PORT` automatically — Toxi reads `$PORT`.

## GKE (Kubernetes)

Deployment manifest:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: toxi-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: toxi-app
  template:
    metadata:
      labels:
        app: toxi-app
    spec:
      containers:
      - name: toxi-app
        image: gcr.io/<project>/toxi-app
        ports:
        - containerPort: 3000
        env:
        - name: TOXI_ENV
          value: "production"
        - name: RUST_LOG
          value: "info"
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
        resources:
          requests:
            cpu: 250m
            memory: 256Mi
          limits:
            cpu: 1
            memory: 512Mi
---
apiVersion: v1
kind: Service
metadata:
  name: toxi-app
spec:
  type: ClusterIP
  ports:
  - port: 80
    targetPort: 3000
  selector:
    app: toxi-app
```

## Compute Engine VM

`startup-script.sh`:
```bash
#!/bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
git clone https://github.com/org/toxi-app.git /opt/toxi-app
cd /opt/toxi-app
cargo build --release
cat > /etc/systemd/system/toxi-app.service << 'EOF'
[Unit]
Description=Toxi App
After=network.target

[Service]
User=toxi
WorkingDirectory=/opt/toxi-app
ExecStart=/opt/toxi-app/target/release/toxi-app
Environment=TOXI_ENV=production
Environment=RUST_LOG=info
Environment=PORT=80
Restart=always

[Install]
WantedBy=multi-user.target
EOF
systemctl enable toxi-app
systemctl start toxi-app
```

## Cloud SQL Setup

```bash
gcloud sql instances create toxi-db \
  --database-version POSTGRES_15 \
  --tier db-custom-2-7680 \
  --region us-central1

gcloud sql databases create toxi --instance toxi-db
gcloud sql users create toxi_admin --instance toxi-db --password <secret>
```

Use the Cloud SQL Auth Proxy for local connections or the Unix socket for GKE/Cloud Run:

```bash
cloud_sql_instances = "<project>:us-central1:toxi-db"
```

Set `DATABASE_URL=postgres://toxi_admin:<secret>@//cloudsql/<instance>/toxi?host=/cloudsql/<instance>`.

## Cloud Load Balancing with WebSocket

Use an external HTTPS load balancer with a backend bucket or NEG. WebSocket support is automatic with HTTP/2. Create a backend service:

```bash
gcloud compute backend-services create toxi-backend \
  --protocol HTTP \
  --port-name http \
  --enable-cdn \
  --timeout 3600
```

Set `idle_timeout_secs` in `toxi.toml` to match the backend timeout.

## Secret Manager

```bash
gcloud secrets create DATABASE_URL --data-file=<(echo -n "postgres://...")
```

Mount secrets in Cloud Run:
```bash
--set-secrets "DATABASE_URL=DATABASE_URL:latest"
```

In GKE, use the Secret Manager CSI driver:
```yaml
volumes:
- name: secrets
  csi:
    driver: secrets-store.csi.k8s.io
    readOnly: true
    volumeAttributes:
      secretProviderClass: toxi-secrets
```

## Cloud Logging Integration

Set `RUST_LOG=info` and write structured JSON logs. Cloud Run and GKE capture `stdout`/`stderr` automatically. For Compute Engine, install the Ops Agent:

```bash
gcloud compute instances add-metadata <instance> \
  --metadata google-logging-enabled=true
```

Use the Toxi logging layer for structured output:
```rust
use toxi_middleware::LoggerLayer;

let app = LoggerLayer::new()
    .with_format("structured")
    .with_request_id(true);
```

## `toxi.toml` — Production Example

```toml
[server]
host = "0.0.0.0"
port = { env = "PORT", default = 3000 }
workers = { env = "TOXI_WORKERS", default = 4 }

[database]
pool_size = 10
connect_timeout_secs = 30

[logging]
format = "json"
level = "info"

[websocket]
max_message_size = 65536
idle_timeout_secs = 600
```
