# notifier — Guide

Queued welcome emails. Shows the job queue plus SMTP mail. API group:
no UI.

## Run

```bash
cd apps/api/notifier
cargo run
```

Listens on `http://localhost:3005`. Delivery needs SMTP:

- `SMTP_HOST`, default `localhost`.
- `SMTP_PORT`, default `1025` (Mailpit default).

## Try it

```bash
cargo test -p notifier

curl -X POST localhost:3005/subscribe \
  -H 'Content-Type: application/json' -d '{"email":"n@n.c","name":"N"}'
curl -X POST localhost:3005/work
curl localhost:3005/stats
```

`/work` runs one queued job now: success completes it, SMTP errors
fail it into the dead-letter list at `/dead-letter`.

## Structure

```
src/main.rs           # wiring only: queue, router, serve
src/routes/subscribe.rs # subscribe, work, stats, dead-letter
src/routes/status.rs  # liveness endpoints
src/jobs/welcome.rs   # WelcomeEmail job plus its SMTP send
```

Every file stays under 500 lines. Responses use `json_response!`;
bodies parse with `Request::json()`.

## Docker

```bash
docker build -t notifier apps/api/notifier
docker run -p 3005:3005 notifier
```
