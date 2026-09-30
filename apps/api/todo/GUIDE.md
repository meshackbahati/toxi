# todo — Guide

Todos over REST, plus the framework GraphQL schema mounted live with
its playground. API group: no UI.

## Run

```bash
cd apps/api/todo
cargo run
```

Listens on `http://localhost:3004`. Environment:

- `TODO_DB` — sqlite URL, default `sqlite:todo.db`.

## Try it

```bash
cargo test -p todo

curl -X POST localhost:3004/todos \
  -H 'Content-Type: application/json' -d '{"text":"buy milk"}'
curl localhost:3004/todos

# GraphQL playground in a browser:
open http://localhost:3004/graphql
# query { apiVersion healthCheck }
```

## Structure

```
src/main.rs           # wiring only: state, router, GraphQL mount, serve
src/routes/todos.rs   # list, create, toggle, delete
src/routes/status.rs  # liveness endpoints
src/models/todo.rs    # Todo and CreateTodo types
migrations/           # todos table
```

Every file stays under 500 lines. Responses use `json_response!`;
bodies parse with `Request::json()`.

## Docker

```bash
docker build -t todo apps/api/todo
docker run -p 3004:3004 todo
```
