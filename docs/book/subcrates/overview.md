# Subcrate Reference Overview

This section documents each Toxi crate, when to use it, and the primary API entry points.

## Core runtime crates

- `toxi`: umbrella crate and prelude
- `toxi-core`: router, request/response, extractors, server
- `toxi-middleware`: reusable HTTP middleware layers
- `toxi-config`: typed application configuration
- `toxi-utils`: utility helpers (ids, strings, validation, dates)

## Data and state crates

- `toxi-db`: ORM and database abstraction
- `toxi-macros`: derive macros (especially `Model`)
- `toxi-cache`: memory/redis caching abstractions
- `toxi-queue`: in-memory/redis/postgres job queues
- `toxi-storage`: local + S3 file storage

## Security and identity crates

- `toxi-auth`: JWT, RBAC, sessions, OAuth helpers
- `toxi-security`: crypto/hash/random/sanitization helpers

## Web/API feature crates

- `toxi-realtime`: websocket/sse/pubsub/event helpers
- `toxi-template`: SSR templates + static file serving
- `toxi-openapi`: OpenAPI spec and docs generation
- `toxi-graphql`: GraphQL schema/handler utilities
- `toxi-mail`: SMTP + message/attachment APIs
- `toxi-plugin`: plugin loading and lifecycle hooks

## Tooling crates

- `toxi-cli`: project generation and developer commands
- `toxi-testing`: test server/request/response helpers
