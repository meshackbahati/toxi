# shortlink — Guide

Cache-first URL shortener. Shows database persistence, in-memory
caching, per-caller rate limiting, and redirect responses. API group:
no UI by design.

## Run

```bash
cd apps/api/shortlink
cargo run
```

Listens on `http://localhost:3001`. Environment:

- `SHORTLINK_DB` — sqlite URL, default `sqlite:shortlink.db`.

## Try it

```bash
cargo test -p shortlink

curl -X POST localhost:3001/links \
  -H 'Content-Type: application/json' -d '{"url":"https://example.com"}'
# {"code":"aB3x9Qz","url":"https://example.com"}

curl -o /dev/null -w "%{http_code} %{redirect_url}\n" localhost:3001/aB3x9Qz
curl localhost:3001/stats
```

Custom codes: `{"url":"...","code":"docs"}`. Taken codes return 409.
Redirects past 60/minute per caller return 429.

## Structure

```
src/main.rs          # wiring only: state, router, serve
src/routes/links.rs  # create, redirect, list, stats
src/routes/status.rs # liveness endpoints
src/models/link.rs   # Link, CreateLink, code generator
migrations/          # links table
```

Every file stays under 500 lines. Responses use `json_response!`;
bodies parse with `Request::json()`.

## Docker

```bash
docker build -t shortlink apps/api/shortlink
docker run -p 3001:3001 shortlink
```
