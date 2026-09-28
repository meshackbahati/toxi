# demo-app — Guide

Showcase application for the Toxi framework. REST JSON, HTML templates,
form posts, cookies, and error pages in one binary.

## Run

```bash
cd apps/demo-app
cargo run
```

The server listens on `http://localhost:3000`. Run from this directory:
templates resolve relative to it.

## Test

```bash
curl -s localhost:3000/api/status
curl -s localhost:3000/users
curl -s localhost:3000/users/1 -w "%{http_code}\n"
curl -s localhost:3000/ -o /dev/null -w "%{http_code}\n"
```

Expected: `/api/status`, `/users` return 200 JSON; `/` renders HTML 200.

## Structure

```
src/main.rs          # wiring only: state, router, serve
src/routes/          # one module per surface (web, auth, api, posts...)
src/models/          # serde types, no logic
src/services/        # shared business logic
templates/           # HTML with layout inheritance
public/              # static assets
migrations/          # SQL schema history
```

Every file stays under 500 lines. When a module approaches the limit,
split it by surface, not by layer.

## Known limitation

`Path<u32>` rejects numeric segments with 400 because route parameters
arrive as strings and deserialization is strict. Prefer `Path<String>`
in handlers and parse inside, until the framework coerces scalars.

## Deploy

Single binary plus `templates/`, `public/`, and `migrations/` beside it.
No runtime beyond the OS. Set `TOXI_ENV=production` to disable the
debug error pages.
