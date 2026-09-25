# toxi-cache

In-memory and Redis caching for Toxi.

```toml
[dependencies]
toxi-cache = "3"
```

```rust
use std::time::Duration;
use toxi_cache::{Cache, MemoryCache};

let cache = MemoryCache::new();
cache.set("user:1", &"Alice", Some(Duration::from_secs(60))).await?;
let user: Option<String> = cache.get("user:1").await?;
```
