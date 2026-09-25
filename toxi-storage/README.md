# toxi-storage

File storage with local disk and S3 backends.

```toml
[dependencies]
toxi-storage = "3"
```

```rust
use bytes::Bytes;
use toxi_storage::{LocalStorage, Storage};

let storage = LocalStorage::new("uploads")?;
storage.put("a.png", Bytes::from_static(b"data")).await?;
let file = storage.get("a.png").await?;
```
