# blog — Guide

Rendered blog with auth-gated writing. Fullstack group: every page is
server-rendered HTML through layout inheritance.

## Run

```bash
cd apps/fullstack/blog
cargo run
```

Listens on `http://localhost:3003`. Environment:

- `BLOG_DB` — sqlite URL, default `sqlite:blog.db`.
- `BLOG_JWT` — signing secret, default `blog-dev-secret`.

## Try it

```bash
cargo test -p blog

curl -X POST localhost:3003/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"b@b.c","password":"password1","name":"B"}'
TOKEN=$(curl -s -X POST localhost:3003/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"b@b.c","password":"password1"}' | python3 -c "import json,sys; print(json.load(sys.stdin)['token'])")
curl -X POST localhost:3003/posts -H "Authorization: Bearer $TOKEN" \
  -H 'Content-Type: application/json' -d '{"title":"Hi","body":"World"}'
curl localhost:3003/
```

## Structure

```
src/main.rs          # wiring only: state, router, serve
src/routes/pages.rs  # GET / and GET /posts/:id rendering
src/routes/write.rs  # register, login, publish
src/routes/status.rs # liveness endpoints
src/models/post.rs   # Post and WritePost types
templates/           # layout.html, index.html, post.html
migrations/          # users plus posts tables
```

Every file stays under 500 lines. Responses use `json_response!`;
bodies parse with `Request::json()`.

## Docker

```bash
docker build -t blog apps/fullstack/blog
docker run -p 3003:3003 blog
```
