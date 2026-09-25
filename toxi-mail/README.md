# toxi-mail

SMTP email sending for Toxi.

```toml
[dependencies]
toxi-mail = "3"
```

```rust
use toxi_mail::{Mailer, Message, SmtpConfig, SmtpTransport};

let transport = SmtpTransport::from_config(
    SmtpConfig::new("smtp.example.com", 587)
        .credentials("user", "pass")
        .use_tls(true),
)?;
let message = Message::new()
    .from("sender@example.com")
    .to("recipient@example.com")
    .subject("Hello")
    .text("Email content");
Mailer::new(transport).send(message).await?;
```
