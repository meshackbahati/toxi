# Installation

This chapter covers how to install Toxi and set up your development environment.

## Prerequisites

Before installing Toxi, you'll need:

- Rust 1.75 or higher
- Cargo (comes with Rust)
- Git

You can install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installing the Toxi CLI

Install the `toxi-cli` package. It provides the `toxi` executable:

```bash
# Install from source (recommended for development)
cargo install --path toxi-cli

# Or install from crates.io
cargo install toxi-cli

# Or pin this generated CLI build
cargo install toxi-cli --version 3.1.1
```

## Creating Your First Project

Once you have the CLI installed, create a new project:

```bash
toxi new my-app
cd my-app
toxi --version
```

This will create a new Toxi project with a basic structure and all necessary dependencies.

## Manual Installation

If you prefer to add Toxi to an existing project manually, add it to your `Cargo.toml`:

```toml
[dependencies]
toxi = { version = "3.2", default-features = false, features = ["database", "auth"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

Enable only the features you import. Core routing, middleware and config are
always included. Note that `features` adds to the defaults, so write
`default-features = false` to skip the rest. `full` turns on everything but
compiles much slower (it pulls the AWS SDK, juniper and the QUIC stack).

## Development Dependencies

For testing and development, you may also want to add:

```toml
[dev-dependencies]
toxi-testing = "2.3"
tokio-test = "0.4"
```

## Verifying Installation

To verify your installation, create a simple test file:

```rust,ignore
use toxi::prelude::*;

async fn hello(_req: Request) -> Result<Response> {
    Ok(Response::text("Hello, Toxi!"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Application::new(());
    app.router_mut().get("/", hello);
    app.run().await;
    // Alternative: Router::new() → router.get(...) → Server::new(router).listen(addr)
}
```

Run this with:

```bash
cargo run
```

You should see your server running on http://127.0.0.1:3000.

## Troubleshooting

If you encounter issues:

1. Ensure you have the latest version of Rust installed
2. Make sure your Cargo is up to date
3. Check that you have all required build tools for your platform
4. Verify that you're using the correct features for your use case

Common features include:
- `full`: All features enabled
- `minimal`: Nothing extra (core routing, middleware, config)
- `database`: Database ORM capabilities
- `auth`: Authentication and authorization
- `queue`: Background job processing
- `cache`: Caching capabilities
- `realtime`: WebSocket and SSE support
- `templates`: Server-side template rendering
- `mail`: Email sending capabilities
- `storage`: File storage (local/S3)
- `graphql`: GraphQL support
- `plugin`: Plugin system support
- `security`: Crypto and hashing
- `utils`: String and date helpers
- `http3`: QUIC/HTTP3 server (slow to compile, off unless asked)
