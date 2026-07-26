# Deploying Toxi on Azure

## App Service (Linux Containers)

1. Build and push to Azure Container Registry:
```bash
az acr build --registry <registry> --image toxi-app:latest .
```
2. Create the App Service:
```bash
az webapp create \
  --resource-group toxi-rg \
  --plan toxi-plan \
  --name toxi-app \
  --deployment-container-image-name <registry>.azurecr.io/toxi-app:latest

az webapp config appsettings set \
  --resource-group toxi-rg \
  --name toxi-app \
  --settings TOXI_ENV=production RUST_LOG=info DATABASE_URL="<connection-string>"
```
App Service sets `PORT=8080` automatically — ensure your binary listens on the port passed via `PORT`.

## AKS (Azure Kubernetes)

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
        image: <registry>.azurecr.io/toxi-app:latest
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
              name: toxi-db
              key: url
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 10
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

Expose via Azure Load Balancer (type: LoadBalancer) or Application Gateway Ingress Controller.

## Azure Container Apps

```bash
az containerapp create \
  --name toxi-app \
  --resource-group toxi-rg \
  --environment toxi-env \
  --image <registry>.azurecr.io/toxi-app:latest \
  --target-port 3000 \
  --ingress external \
  --min-replicas 1 \
  --max-replicas 10 \
  --env-vars "TOXI_ENV=production" "RUST_LOG=info" \
  --secrets "db-url=<connection-string>" \
  --secret-volume-mount "/mnt/secrets"
```

Container Apps support automatic HTTP/1.1 → HTTP/2 upgrades for WebSocket.

## Azure Database for PostgreSQL / MySQL

```bash
az postgres flexible-server create \
  --name toxi-db \
  --resource-group toxi-rg \
  --sku-name Standard_B1ms \
  --admin-user toxi_admin \
  --admin-password <secret> \
  --public-access 0.0.0.0

az postgres flexible-server db create \
  --server-name toxi-db \
  --database-name toxi
```

Set `DATABASE_URL=postgres://toxi_admin:<secret>@toxi-db.postgres.database.azure.com:5432/toxi?sslmode=require`.

## Application Gateway with WebSocket

Application Gateway supports WebSocket natively. Configure:
```bash
az network application-gateway probe create \
  --gateway-name toxi-gw \
  --name health-probe \
  --protocol Http \
  --path /health \
  --interval 30 \
  --timeout 10
```

Enable WebSocket (`--enable-ws true`) on the HTTP listener. No additional Toxi-side configuration is needed.

## Key Vault for Secrets

```bash
az keyvault create --name toxi-kv --resource-group toxi-rg
az keyvault secret set --vault-name toxi-kv --name "DATABASE-URL" --value "<connection-string>"
```

Use managed identity to access Key Vault from App Service:
```bash
az webapp config appsettings set \
  --name toxi-app \
  --resource-group toxi-rg \
  --settings "DATABASE_URL=@Microsoft.KeyVault(SecretUri=https://toxi-kv.vault.azure.net/secrets/DATABASE-URL/)"
```

For AKS, use the Azure Key Vault Secrets Store CSI driver.

## Application Insights Monitoring

```bash
az monitor app-insights component create \
  --app toxi-insights \
  --resource-group toxi-rg \
  --location eastus
```

Set `APPLICATIONINSIGHTS_CONNECTION_STRING` as an environment variable. Configure Toxi logging to emit JSON and integrate via the Application Insights SDK or a sidecar agent:

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
workers = 4
backlog = 1024

[database]
pool_size = 10
connect_timeout_secs = 30
sslmode = "require"

[logging]
format = "json"
level = "info"

[websocket]
max_message_size = 65536
idle_timeout_secs = 300
```
