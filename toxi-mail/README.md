# toxi-mail

SMTP email sending for Toxi.

## Installation

```toml
[dependencies]
toxi-mail = "2.3.4"
```

## Basic Usage

```rust
use toxi_mail::{Mailer, Message, SmtpConfig, SmtpTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = SmtpConfig::new("smtp.example.com", 587)
        .credentials("smtp-user", "smtp-pass")
        .use_tls(true);

    let transport = SmtpTransport::from_config(config)?;
    let mailer = Mailer::new(transport);

    let message = Message::new()
        .from("sender@example.com")
        .to("recipient@example.com")
        .subject("Hello")
        .text("Email content");

    mailer.send(message).await?;
    Ok(())
}
```

## Attachments

```rust
use toxi_mail::{Attachment, Message};

let message = Message::new()
    .from("sender@example.com")
    .to("recipient@example.com")
    .subject("With attachment")
    .html("<img src=\"cid:logo\" />")
    .attach(
        Attachment::from_file("./logo.png")?
            .inline_with_cid("logo")
    );
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Notes

- `Message` validates required fields and email addresses before send.
- Use `mailer.verify().await` to test SMTP connectivity.
