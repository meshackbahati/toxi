# CLI Tools

The CLI package name is `toxi-cli`. The installed executable is `toxi`.

## Installation

```bash
# Install from crates.io
cargo install toxi-cli

# Install from this repository
cargo install --path toxi-cli
```

## What the CLI Actually Does

The CLI is a **code generation and project management tool**. It:

- Creates project scaffolding with standard structure
- Generates boilerplate code (models, routes, controllers, migrations)
- Runs and manages database migrations
- Starts dev servers with file watching
- Manages background processes

It does **not**:

- Compile your code faster than `cargo build`
- Prevent you from writing bad code
- Replace understanding of Rust, cargo, or the underlying libraries

## Project Creation

```bash
toxi new my-project
toxi new my-api --project-type api
toxi new my-web --template web
```

Generated projects include a standard directory structure. You can reorganize it, but the CLI tools expect this layout.

```text
my-project/
├── Cargo.toml
├── toxi.toml
├── migrations/
├── src/
│   ├── main.rs
│   ├── models/
│   ├── routes/
│   └── controllers/
└── tests/
```

## Code Generators

Generators create files with boilerplate code. They won't overwrite existing files without asking.

```bash
toxi generate model User
toxi generate model User email:string age:integer
toxi generate route users
toxi generate controller UserController
toxi generate middleware AuthMiddleware
```

**Supported generators:**

| Generator | Creates | Notes |
|-----------|---------|-------|
| `model` | Struct with `#[derive(Model)]` | Generates CRUD methods, queries, validation |
| `route` | Route module | Basic router with handlers |
| `controller` | Controller struct | REST-style endpoint handlers |
| `middleware` | Middleware impl | Tower-style middleware |
| `service` | Service struct | Business logic layer |
| `validator` | Validator struct | Request validation |
| `job` | Background job | Queue job handler |
| `policy` | Authorization policy | RBAC/PBAC rules |
| `event` | Domain event | Event struct + handlers |
| `migration` | SQL migration file | Template with up/down sections |
| `seeder` | Database seeder | Seed data script |

**Model field types:**

`string`, `text`, `integer`, `float`, `decimal`, `boolean`, `uuid`, `json`, `timestamp`

The generated code is yours to modify. The generators are one-time scaffolding tools.

## Migrations

```bash
# Create and run
toxi migrate create create_users_table
toxi migrate run

# Check status and rollback
toxi migrate status
toxi migrate revert

# Auto-generate from model changes
toxi make-migrations
toxi make-migrations add_email_field --dry-run
```

Migration files are plain SQL:

```sql
-- migrate:up
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL
);

-- migrate:down
DROP TABLE users;
```

**Limitations:**
- Migrations are tracked by filename, not content hash
- No auto-detection of destructive changes (dropping columns)
- Rollback only reverts the last migration, not arbitrary ones
- `make-migrations` generates SQL based on model differences but may miss complex schema changes

## Database Seeders

```bash
toxi seed create users_seed
toxi seed run
```

Seeders run your code against the database. You write the insertion logic yourself.

## Queue Management

```bash
toxi queue work --workers 4
toxi queue list
toxi queue dlq
toxi queue clear
```

Queues require Redis for production. The in-memory backend is for development only and loses data on restart.

## Development Server

```bash
toxi dev
toxi dev --port 8080
toxi dev --watch src --watch templates
toxi dev --no-hot-reload
```

The dev server:
- Builds once with `cargo build --bin <name>`, then runs the binary directly
- Watches `src/`, `migrations/`, `seeds/`, `templates/`, `tests` plus `Cargo.toml`, `Cargo.lock`, `toxi.toml`, `.env` (override with `--watch` and `--ignore`)
- Rebuilds in the background on content changes and keeps the old server up until the new binary is ready, then swaps with SIGTERM and a 5s grace period
- Picks up changes that land mid-build instead of dropping them
- Resolves the binary from the `[package] name` in Cargo.toml (override with `--bin`)
- A `touch` with no content change does not trigger a rebuild
- Failed builds keep the old server running

## Run Single Files

Execute a Rust file directly without creating a full project:

```bash
# Standalone file (creates temp project, runs, cleans up)
toxi run hello.rs

# File inside project (copies to src/bin/)
toxi run src/bin/script.rs

# With extra dependencies
toxi run api.rs --deps serde,chrono
```

**How it works:**
- **Standalone mode**: Creates a temporary Cargo project in `/tmp`, compiles with toxi dependencies, runs, then deletes the temp directory
- **Project mode**: Places the file in `src/bin/` and runs via `cargo run --bin`
- Compile errors display directly in your terminal
- Not meant for production - use `toxi build` for deployable binaries

## Process Management

Manage long-running toxi processes:

```bash
toxi pm2 start
toxi pm2 start my-api --release
toxi pm2 stop my-api
toxi pm2 restart my-api
toxi pm2 list
toxi pm2 info my-api
toxi pm2 monitor
```

Process state is stored in `.toxi_procs.json` in the current directory. This is **not** a replacement for systemd, Docker, or proper process managers in production.

## Debug Logging

Enable verbose output:

```bash
TOXI_DEBUG=1 toxi dev
TOXI_DEBUG=true toxi serve
```

Shows environment loading, configuration details, file paths, and internal operations.

## Colored Output

Errors display in red, success in green, warnings in yellow, info in blue. Error messages are categorized by type (compile errors, runtime errors, permission errors).

## Build and Deploy

```bash
toxi build --release
toxi serve --env production
```

`toxi build` runs `cargo build` with your specified flags. It doesn't optimize beyond what cargo already does.

## Diagnostics

```bash
toxi doctor
```

Checks for:
- Rust and Cargo installation
- Required project files (`Cargo.toml`, `toxi.toml`)
- Migration directory
- Common environment variables

It reports what it finds but doesn't fix issues automatically.
