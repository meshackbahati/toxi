//! A welcome-email job. Payload carries the recipient; `perform`
//! rebuilds the message and sends it through SMTP.

use toxi::mail::{Mailer, Message, SmtpTransport};
use toxi_queue::{Job, JobResult};

/// Send a welcome email to a new address.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WelcomeEmail {
    pub email: String,
    pub name: String,
}

#[async_trait::async_trait]
impl Job for WelcomeEmail {
    async fn perform(&self) -> JobResult {
        let host = std::env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port: u16 = std::env::var("SMTP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(1025);
        let transport = SmtpTransport::new(host, port)
            .map_err(|e| toxi_queue::QueueError::JobFailed(e.to_string()))?;
        let mailer = Mailer::new(transport);
        let message = Message::new()
            .from("welcome@example.com")
            .to(self.email.clone())
            .subject(format!("Welcome, {}!", self.name))
            .text(format!("Hi {}, thanks for joining.", self.name));
        mailer
            .send(message)
            .await
            .map_err(|e| toxi_queue::QueueError::JobFailed(e.to_string()))?;
        Ok(())
    }
}
