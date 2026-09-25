# toxi-auth

JWT, passwords, RBAC, API keys, 2FA, and OAuth2 for Toxi.

```toml
[dependencies]
toxi-auth = "3"
```

```rust
use toxi_auth::{JwtManager, create_token, verify_token, Claims};

let manager = JwtManager::new("secret".to_string());
let token = create_token(&manager, Claims {
    sub: "user-1".to_string(),
    ..Default::default()
})?;
let claims = verify_token(&manager, &token)?;
```
