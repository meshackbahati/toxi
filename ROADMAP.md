# Toxi Roadmap

This roadmap tracks technical implementation work in this repository.

## Current release target

- Framework release line: `2.3.5`
- Stability: `beta`
- Source of truth: this file (`ROADMAP.md`)

## Completed Rename (v2.3.5) — COMPLETE

- [X] **Rename to Toxi**: Complete project rename from **Oxidite** to **Toxi** — all crates, directories, env vars, config files, docs, and code references.

## Completed WebSocket Fix (v2.3.5) — COMPLETE

- [X] **HTTP/2 ALPN Negotiation**: Server now negotiates HTTP/2 via ALPN when TLS is used.
- [X] **Proxy Configuration Docs**: Added nginx/ALB/Cloudflare WebSocket proxy setup guide.
- [X] **Connection Error Logging**: Improved error logging to distinguish HTTP/2 vs HTTP/1.1 parser issues.

## Completed Security Fix (v2.3.5) — COMPLETE

- [X] **SQL Injection in Savepoints**: Added identifier validation for savepoint names.

## Completed Code Quality (v2.3.5) — COMPLETE

- [X] **Handler Impl Ordering**: Reordered Handler trait impls to sequential order.
- [X] **CORS Double-Header Fix**: Removed duplicate CORS header addition from Router level.
- [X] **Config Env Var Names**: Updated env vars from `OXIDITE_*` to `TOXI_*`.

## Completed Enterprise Scalability (v2.3.0 Stabilization) — COMPLETE

- [X] **Expanded Handler Capacity**: Support for up to 12 extractors per handler.
- [X] **Global Router Layering**: Unified middleware application via `Router::layer()`.
- [X] **Authenticated WebSockets**: Context preservation during protocol upgrades.
- [X] **Raw SQL Access**: Public accessors for underlying `sqlx` pools.
- [X] **Universal Primary Keys**: `Uuid` and `String` support in `Model` derive.

## Completed modernization stream (v2.2.0 Hardening) — COMPLETE

- [X] **Advanced ORM Validation**: Async validations for Models (`length`, `range`, `email`, `url`, `regex`, `custom`, `unique`).
- [X] **N+1 Eager Loading**: Batch `IN` queries support in derive macros (`eager_load_posts`, `eager_load_profile`) alongside lazy-loading relations.
- [X] **Unified Cloud Storage**: Complete `StorageFacade` supporting Local, S3, Cloudinary, and ImageKit backends.
- [X] **Ignition-style Diagnostics**: Rich HTML trace pages for development-mode 500 exceptions.
- [X] **Interactive REPL (`toxi tinker`)**: Full cargo-integrated interactive console CLI command.
- [X] **Compile-Time Router Verification**: Added `IntoHandler` trait and `handler_fn` route helper to verify extractors at compile time.
- [X] **State Injection DX**: Scaffolded controllers and generators to use `State<Arc<AppState>>` out of the box.

## Batch A (v1.1 carry-over)

- [X] WebSocket presence tracking (`toxi-realtime`)
- [ ] Advanced monitoring and metrics (`toxi-core`, `toxi-middleware`, `toxi-utils`)
- [ ] Performance profiling tools (`toxi-cli`, `toxi-testing`)
- [ ] Deployment guides (AWS/GCP/Azure docs)
- [ ] Migration guide from other frameworks (`docs/` + examples)
- [ ] Example fullstack apps using TemplateContext and template files (`examples/`)

## Batch B (v2.1 performance/scalability) — COMPLETE

- [X] Zero-copy oriented request/response path improvements (`toxi-core`)
- [X] Async streaming response support (`toxi-core`)
- [X] Connection pooling optimizations (`toxi-db`)
- [X] Database connection multiplexing patterns and transaction ergonomics (`toxi-db`)
- [X] Enhanced testing framework APIs (`toxi-testing`)
- [X] Mock server support (`toxi-testing`)
- [X] Integration testing helpers (`toxi-testing`, `toxi-cli`)
- [X] Benchmark tooling baseline (`toxi-testing` + bench utilities)

Batch B completion marker date: `2026-03-29`.

## Batch C (v3 engineering-only subset)

- [ ] API gateway functionality
- [ ] Audit logging
- [ ] Compliance report generation primitives
- [ ] Multi-region deployment tooling primitives
- [ ] Disaster recovery tooling primitives

## Notes

- Prior roadmap entries (pre-v2.3.5) refer to the framework under its former name **Oxidite**.
- Batch A items remain the next priority after v2.3.5 stabilization.
- Batch C items are deferred to v3 planning phase.
