# CLI Tool Guide

The CLI package is `toxi-cli`. The installed binary is `toxi`.

## Installation

```bash
cargo install toxi-cli
```

## Project Creation

```bash
toxi new myapp
toxi new myapp --type api
toxi new myapp --template web
```

Generated projects include:

- `toxi.toml`
- `migrations/`
- `seeds/`
- `src/controllers/`
- `src/events/`
- `src/jobs/`
- `src/middleware/`
- `src/models/`
- `src/policies/`
- `src/routes/`
- `src/services/`
- `src/validators/`

## Generators

Use `generate` for new workflows. `make` remains as a hidden compatibility alias.

```bash
toxi generate model User
toxi generate model User email:string age:integer
toxi generate route users
toxi generate controller UserController
toxi generate middleware AuthMiddleware
toxi generate service Billing
toxi generate validator CreateUser
toxi generate job SendDigest
toxi generate policy Post
toxi generate event UserSignedUp
toxi generate migration create_users_table
toxi generate seeder users_seed
```

## Migrations

```bash
toxi migrate create create_users_table
toxi migrate
toxi migrate run
toxi migrate status
toxi migrate revert
toxi migrate:rollback
```

Migration files use the SQL sections the runtime understands:

```sql
-- migrate:up
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL
);

-- migrate:down
DROP TABLE users;
```

## Seeders

```bash
toxi seed create users_seed
toxi seed
toxi seed run
toxi db:seed
```

## Queue Commands

Canonical commands:

```bash
toxi queue work --workers 4
toxi queue list
toxi queue dlq
toxi queue clear
```

Compatibility aliases:

```bash
toxi queue:work --workers 4
toxi queue:list
toxi queue:dlq
toxi queue:clear
```

## Development

```bash
toxi dev
toxi dev --port 8080
toxi dev --host 0.0.0.0 --env development
toxi dev --watch src --watch templates
toxi dev --ignore dist
toxi dev --no-hot-reload
```

The CLI forwards host, port, and environment overrides through:

- `SERVER_HOST`
- `SERVER_PORT`
- `TOXI_ENV`

## Build And Serve

```bash
toxi build
toxi build --release
toxi build --profile release
toxi build --target x86_64-unknown-linux-musl
toxi build --features "database,queue"
toxi build --verbose

toxi serve
toxi serve --addr 0.0.0.0:8080
toxi serve --env production
```

## Diagnostics

```bash
toxi doctor
toxi --help
toxi --version
toxi version
```
