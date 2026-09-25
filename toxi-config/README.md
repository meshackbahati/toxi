# toxi-config

Loads `toxi.toml`, `.env`, and OS environment variables into typed
config sections.

```toml
[dependencies]
toxi-config = "3"
```

```rust
use toxi_config::Config;

let config = Config::load().unwrap_or_default();
let port: u16 = config.get("server.port").unwrap_or(3000);
```
