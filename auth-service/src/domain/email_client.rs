use super::Email;
use color_eyre::eyre::Result;

#[async_trait::async_trait]
// pub trait EmailClient {
pub trait EmailClient: std::fmt::Debug {
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()>;
}
