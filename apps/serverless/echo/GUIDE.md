# echo — Guide

Serverless-style webhook: one stateless handler plus a warmth probe.
Serverless group: no state, no UI, cold-start fast.

## Run

```bash
cd apps/serverless/echo
cargo run
```

Listens on `http://localhost:3002`.

## Try it

```bash
cargo test -p echo

curl -X POST localhost:3002/hook \
  -H 'Content-Type: application/json' -d '{"hello":"world"}'
# {"echo":{"hello":"world"},"at":"..."}
```

## Structure

```
src/main.rs           # wiring only: router, serve
src/routes/hook.rs    # POST /hook handler plus its test
src/routes/health.rs  # GET /health warmth probe
```

One concern per file so each piece reads alone. Bodies parse with
`Request::json()`; responses use `json_response!`.

## Docker

```bash
docker build -t echo apps/serverless/echo
docker run -p 3002:3002 echo
```
