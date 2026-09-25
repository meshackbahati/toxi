# toxi-macros

Procedural macros for Toxi. Currently just `#[derive(Model)]`.

```toml
[dependencies]
toxi-macros = "3"
```

```rust
use toxi_db::Model;

#[derive(Model)]
#[model(table = "users")]
struct User {
    id: i64,
    name: String,
    #[validate(email)]
    email: String,
}
```
