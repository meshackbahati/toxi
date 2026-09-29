# toxi-apps

<div align="center">

<img src="docs/logo/toxi.svg" width="200" alt="Toxi Logo">

Working applications built with the [Toxi](https://github.com/toxi-rs/toxi)
web framework, plus the framework documentation hub.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache-2.0-blue.svg)](LICENSE)

</div>

---

## Applications

| App | What it shows | Guide |
| --- | ------------ | ----- |
| [taskboard](apps/taskboard) | Tasks with auth, sqlite, uploads, events, templates, OpenAPI | [GUIDE.md](apps/taskboard/GUIDE.md) |

Every app depends on the published crates, follows the modular
`routes / models / services` layout, and keeps each file under 500 lines.
Each app ships its own guide with run, test, structure, and deploy notes.

## Framework crates

All crates live in the [`toxi-rs`](https://github.com/toxi-rs)
organization, one repo per crate: `toxi`, `toxi-core`, `toxi-cli`,
`toxi-auth`, `toxi-cache`, `toxi-config`, `toxi-db`, `toxi-graphql`,
`toxi-middleware`, `toxi-openapi`, `toxi-plugin`, `toxi-queue`,
`toxi-realtime`, `toxi-security`, `toxi-storage`, `toxi-template`,
`toxi-testing`, `toxi-utils`, `toxi-macros`, `toxi-mail`.

```toml
[dependencies]
toxi = "3"
tokio = { version = "1", features = ["full"] }
```

## Documentation

- [`docs/`](docs) — framework guides: getting started, core concepts,
  authentication, database, middleware, deployment, migrations.
- [`examples/`](examples) — single-file code samples.

## Run an app

```bash
cd apps/taskboard
cargo run
```
