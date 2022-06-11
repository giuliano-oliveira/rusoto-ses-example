mod email;
use async_trait::async_trait;
pub use email::Email;
use rusoto_ses::{Ses, SesClient};
use std::convert::Into;
use traits::AsyncSender;

pub struct SesMailer {
    client: SesClient,
}

impl SesMailer {
    pub fn new(client: SesClient) -> Self {
        Self { client }
    }
}

#[derive(Debug)]
pub enum SesEmailSendError {
    EmailSendError(String),
}

#[async_trait]
impl AsyncSender<Email> for SesMailer {
    type Error = SesEmailSendError;
    async fn send(&self, email: &Email) -> Result<(), Self::Error> {
        self.client
            .send_email(email.into())
            .await
            .map_err(|e| return Self::Error::EmailSendError(e.to_string()))?;
        Ok(())
    }
}
