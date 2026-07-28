# toxi-storage

File storage abstraction for Toxi with local filesystem and optional S3 backend.

## Installation

```toml
[dependencies]
toxi-storage = "3.1.0"
```

Disable S3 if you only need local storage:

```toml
[dependencies]
toxi-storage = { version = "3.1.0", default-features = false }
```

## Usage

```rust
use bytes::Bytes;
use toxi_storage::{LocalStorage, Storage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = LocalStorage::new("uploads")?;

    storage.put("avatars/user-1.png", Bytes::from_static(b"png-data")).await?;
    let file = storage.get("avatars/user-1.png").await?;
    assert!(!file.is_empty());

    Ok(())
}
```

## Validation

Use `FileValidator` + `ValidationRules` to enforce upload constraints:

```rust
use bytes::Bytes;
use toxi_storage::{FileValidator, ValidationRules};

let rules = ValidationRules::new()
    .max_size(5 * 1024 * 1024)
    .allowed_extensions(vec!["png".into(), "jpg".into()])
    .allowed_mime_types(vec!["image/".into()]);

let validator = FileValidator::new(rules);
validator.validate("avatar.png", &Bytes::from_static(b"data"))?;
# Ok::<(), toxi_storage::StorageError>(())
```
