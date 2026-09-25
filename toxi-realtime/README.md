# toxi-realtime

WebSockets, rooms, pub/sub, and SSE helpers for Toxi.

```toml
[dependencies]
toxi-realtime = "3"
```

```rust
use toxi_realtime::{Message, PubSub, WebSocketManager};

let pubsub = PubSub::new();
let mut sub = pubsub.subscribe("news").await;
pubsub.publish("news", Message::text("hi")).await?;
let _ = sub.recv().await;
```
