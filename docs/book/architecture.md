# Architecture

This chapter explains how Toxi is split across crates and how requests move through the system.

## Workspace Structure

- `toxi`: top-level facade and feature flags.
- `toxi-core`: router, request/response, server primitives.
- `toxi-middleware`: common cross-cutting layers.
- `toxi-db` + `toxi-macros`: ORM, derive macros, migrations.
- `toxi-auth`, `toxi-cache`, `toxi-queue`, `toxi-realtime`, `toxi-template`: batteries-included runtime capabilities.
- `toxi-cli`: scaffolding, migration, and developer workflow tooling.

## Request Lifecycle

1. The server accepts an HTTP request in `toxi-core`.
2. The router matches method/path and prepares extractors.
3. Middleware chain runs pre-handler logic.
4. Handler executes with typed extractors.
5. Handler returns typed response.
6. Middleware chain runs post-handler logic.
7. Response is serialized and returned to the client.

## Database Layer Design

Toxi ORM sits on top of `sqlx::Any`:

- `Database` trait abstracts pool/transaction execution.
- `Model` trait provides typed CRUD and validation hooks.
- `ModelQuery` offers builder ergonomics.
- Relationship helpers (`HasMany`, `HasOne`, `BelongsTo`) keep joins and loading explicit.
- Raw SQL remains first-class through `execute_query`/`fetch_all`/`fetch_one`.

## Extension Strategy

Prefer adding capabilities in dedicated crates and surfacing stable public APIs through `toxi`.

This keeps compile times predictable and avoids making core crates monolithic.
