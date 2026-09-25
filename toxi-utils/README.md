# toxi-utils

Small helpers: dates, IDs, strings, validation, metrics.

```toml
[dependencies]
toxi-utils = "3"
```

```rust
use toxi_utils::{generate_uuid, is_email, slugify};

let id = generate_uuid();
assert!(is_email("user@example.com"));
assert_eq!(slugify("Hello World!"), "hello-world");
```
