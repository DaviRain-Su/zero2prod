#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Fake;

    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use zero2prod::domain::SubscriberEmail;
    use zero2prod::email_client::EmailClient;

    #[tokio::test]
    #[ignore]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email: String = SafeEmail().fake();
        let sender = SubscriberEmail::parse(&email).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender);
        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;
        let email: String = SafeEmail().fake();
        let subscriber_email = SubscriberEmail::parse(&email).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();
        // Act
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;
        // Assert
    }
}
