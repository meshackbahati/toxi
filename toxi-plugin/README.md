# toxi-plugin

Plugins with lifecycle hooks for Toxi apps.

```toml
[dependencies]
toxi-plugin = "3"
```

```rust
use toxi_plugin::{Plugin, PluginInfo, PluginResult};
use toxi::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "hello".to_string(),
            version: "1.0.0".to_string(),
            author: "you".to_string(),
            description: "says hello".to_string(),
        }
    }

    fn init(&self, app: &mut Router) -> PluginResult<()> {
        app.get("/hello", |_: Request| async {
            Ok(response::text("Hello from plugin!"))
        });
        Ok(())
    }
}
```
