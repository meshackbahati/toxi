# Research Brief: Toxi Framework Improvements

**Date**: 2026-07-26
**Question**: How can the Toxi Rust web framework be improved? What new features, patterns, and capabilities should it adopt?

## Context

Toxi is a modular Rust web framework built on hyper/tokio with:
- Regex-based routing with typed extractors (like Axum)
- Custom ORM with #[derive(Model)] (like SeaORM/Diesel but custom)
- JWT/OAuth2/RBAC auth
- WebSocket/SSE realtime
- Background jobs (Postgres/Redis)
- Cache (memory/Redis)
- Storage (local/S3)
- Templates, mail, OpenAPI, GraphQL, plugins
- CLI with scaffolding, tinker REPL, dev server
- HTTP/1.1 + HTTP/2 support
- Tower middleware compatibility

## Scope

Research what modern Rust web frameworks offer that Toxi doesn't, and what emerging patterns/tools it should adopt. Focus on:

1. What features do competing Rust frameworks (Axum, Actix, Rocket, Loco, Poem, Salvo) have that Toxi lacks?
2. What are the latest Rust web ecosystem developments (2025-2026)?
3. What developer experience improvements are trending?
4. What production-readiness features are expected?
5. What makes a Rust web framework successful in the ecosystem?

## Depth: standard
