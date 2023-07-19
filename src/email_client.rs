use crate::domain::SubscriberEmail;
use anyhow::Result;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct EmailClient {
    pub http_client: Client,
    pub base_url: String,
    pub sender: SubscriberEmail,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
        }
    }

    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<()> {
        todo!()
    }
}
