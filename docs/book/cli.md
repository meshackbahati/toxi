# CLI Tools

The Toxi CLI package is `toxi-cli`, and the installed executable is `toxi`.

## Installation

```bash
cargo install toxi-cli
```

## What the CLI Does

The CLI generates code, manages projects, and runs development tools. It saves typing but doesn't replace understanding of the underlying technologies.

**It does:**
- Create project scaffolding
- Generate boilerplate models, routes, controllers
- Run database migrations
- Start development servers with file watching
- Execute single Rust files without full project setup
- Manage background processes

**It doesn't:**
- Speed up compilation beyond cargo's capabilities
- Prevent architectural mistakes
- Replace knowledge of Rust, sqlx, or hyper

## Project Scaffolding

```bash
toxi new my_app
toxi new my_api --project-type api
toxi new my_web --template web
```

Generated structure includes standard directories. The CLI tools expect this layout for commands like `toxi generate` and `toxi migrate`.

## Code Generation

```bash
toxi generate model User email:string age:integer
toxi generate route users
toxi generate controller UserController
```

Generated code is starting point boilerplate. You modify it to fit your needs. Generators don't overwrite existing files.

## Database Migrations

```bash
toxi migrate create create_users_table
toxi migrate run
toxi migrate status
toxi migrate revert
toxi make-migrations  # Auto-generate from model changes
```

Migrations are SQL files with `-- migrate:up` and `-- migrate:down` sections. The migration tracker stores which migrations have run, not their content hashes.

## Development Server

```bash
toxi dev
toxi dev --port 8080 --watch src
```

Builds the binary once, runs it, and rebuilds on file changes. Watches source
and config paths, swaps the running server after a successful build, keeps the
old one up if the build fails. A `touch` with no content change does not
trigger a rebuild. Compile times are the same as manual `cargo build`.

## Single File Execution

```bash
toxi run script.rs
toxi run script.rs --deps serde,chrono
```

Creates temp projects for standalone files. Useful for scripts and prototypes, not for production code.

## Process Management

```bash
toxi pm2 start --release
toxi pm2 list
toxi pm2 stop <name>
```

Stores process state locally. For production deployment, use systemd, Docker, or Kubernetes instead.

## Debug and Output

```bash
TOXI_DEBUG=1 toxi dev
```

Colored output: red for errors, green for success, yellow for warnings, blue for info. Errors are categorized by type automatically.
