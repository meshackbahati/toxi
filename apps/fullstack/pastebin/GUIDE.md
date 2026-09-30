# pastebin — Guide

Store raw pastes, serve sanitized previews. Shows the security crate
plus rendered pages. Fullstack group: the index form posts from the page.

## Run

```bash
cd apps/fullstack/pastebin
cargo run
```

Listens on `http://localhost:3006`. Environment:

- `PASTEBIN_DB` — sqlite URL, default `sqlite:pastebin.db`.

## Try it

```bash
cargo test -p pastebin
```

Open `http://localhost:3006`, paste `<script>alert(1)</script>hi`, save.
The preview shows `hi`; `/pastes/:id/raw` shows the escaped original.

## Structure

```
src/main.rs             # wiring only: state, router, serve
src/routes/pastes.rs    # list, create, view, raw
src/routes/status.rs    # liveness endpoints
src/models/paste.rs     # Paste and CreatePaste types
templates/              # layout, index with form, paste view
migrations/             # pastes table
```

Every file stays under 500 lines. Responses use `json_response!`;
bodies parse with `Request::json()`.

## Docker

```bash
docker build -t pastebin apps/fullstack/pastebin
docker run -p 3006:3006 pastebin
```
