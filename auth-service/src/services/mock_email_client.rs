use crate::domain::{Email, EmailClient};
use color_eyre::eyre::Result;
use secrecy::ExposeSecret;
// use secrecy::ExposeSecret;

#[derive(Debug)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    #[tracing::instrument(
        name = "Sending email",
        skip(self, recipient, subject, content),
        fields(
            recipient = %recipient.as_ref().expose_secret(),
            subject = %subject,
            content_length = content.len()
        )
    )]
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        tracing::info!(
            "Sending email to {} with subject: {} and content: {}",
            recipient.as_ref().expose_secret(),
            subject,
            content
        );

        Ok(())
    }
}
