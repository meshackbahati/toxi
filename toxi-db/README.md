# toxi-db

Small ORM and file migrations on `sqlx`, with `#[derive(Model)]`.

```toml
[dependencies]
toxi-db = "3"
```

```rust
use toxi_db::{DbPool, Model};

#[derive(Model)]
#[model(table = "users")]
struct User {
    id: i64,
    name: String,
    email: String,
}

let db = DbPool::connect("sqlite::memory:").await?;
let user = User::find_by_id(&db, 1).await?;
```
