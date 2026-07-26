# Migrating from Oxidite 2.x to Toxi 3.0

Oxidite has been renamed to **Toxi**. This guide covers the changes you need to make to migrate an existing Oxidite 2.x project to Toxi 3.0.

## Dependency changes

Replace all `oxidite-*` crate dependencies with their `toxi-*` equivalents:

```toml
# Before (Cargo.toml)
[dependencies]
oxidite = "2.3"
oxidite-core = "2.3"
oxidite-cli = "2.3"
oxidite-db = "2.3"
oxidite-auth = "2.3"

# After
[dependencies]
toxi = "3.0"
toxi-core = "3.0"
toxi-cli = "3.0"
toxi-db = "3.0"
toxi-auth = "3.0"
```

## Import paths

Replace `oxidite::` with `toxi::` in all `use` statements:

```rust
// Before
use oxidite::prelude::*;
use oxidite::oxidite_core::Response;

// After
use toxi::prelude::*;
use toxi::toxi_core::Response;
```

## Crate names

| Old (Oxidite) | New (Toxi) |
|---|---|
| `oxidite` | `toxi` |
| `oxidite-core` | `toxi-core` |
| `oxidite-cli` | `toxi-cli` |
| `oxidite-db` | `toxi-db` |
| `oxidite-auth` | `toxi-auth` |
| `oxidite-realtime` | `toxi-realtime` |
| `oxidite-middleware` | `toxi-middleware` |
| `oxidite-config` | `toxi-config` |
| `oxidite-cache` | `toxi-cache` |
| `oxidite-queue` | `toxi-queue` |
| `oxidite-template` | `toxi-template` |
| `oxidite-mail` | `toxi-mail` |
| `oxidite-storage` | `toxi-storage` |
| `oxidite-security` | `toxi-security` |
| `oxidite-utils` | `toxi-utils` |
| `oxidite-openapi` | `toxi-openapi` |
| `oxidite-graphql` | `toxi-graphql` |
| `oxidite-testing` | `toxi-testing` |
| `oxidite-plugin` | `toxi-plugin` |
| `oxidite-macros` | `toxi-macros` |

## Environment variables

Rename all `OXIDITE_*` environment variables to `TOXI_*`:

| Before | After |
|---|---|
| `OXIDITE_ENV` | `TOXI_ENV` |
| `OXIDITE_DATABASE_URL` | `TOXI_DATABASE_URL` |
| `OXIDITE_JWT_SECRET` | `TOXI_JWT_SECRET` |
| `OXIDITE_REDIS_URL` | `TOXI_REDIS_URL` |
| `OXIDITE_STORAGE_BACKEND` | `TOXI_STORAGE_BACKEND` |
| `OXIDITE_LOG_LEVEL` | `TOXI_LOG_LEVEL` |
| `OXIDITE_CONFIG` | `TOXI_CONFIG` |

## Config files

Rename `oxidite.toml` to `toxi.toml` in your project root. The internal structure is unchanged.

## CLI usage

Replace `oxidite` with `toxi` when using the CLI:

```bash
# Before
oxidite new my-app
oxidite generate model User
oxidite migrate

# After
toxi new my-app
toxi generate model User
toxi migrate
```

The installed binary is now `toxi`. Install it with:

```bash
cargo install toxi-cli --version 3.0
```

## Version

Update your version references from `2.x` to `3.0`:

```toml
[dependencies]
toxi = "3.0"
```

## What hasn't changed

- The API for routing, handlers, extractors, middleware, and ORM operations is the same.
- `TemplateContext`, `Application`, `Router`, `Request`, `Response`, and other core types work identically.
- The CLI commands (`new`, `generate`, `migrate`, `dev`, `build`) have the same flags and behaviour.
- The config file format (`toxi.toml`) is the same as the old `oxidite.toml`.

## Need help?

Open an issue at [github.com/meshackbahati/toxi](https://github.com/meshackbahati/toxi).
