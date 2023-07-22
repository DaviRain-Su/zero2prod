#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};

    use secrecy::Secret;
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use zero2prod::domain::SubscriberEmail;
    use zero2prod::email_client::EmailClient;

    use wiremock::Request;

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, _request: &Request) -> bool {
            // try to parse the body as as JSON value
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&_request.body);

            if let Ok(body) = result {
                dbg!(&body);
                // check if the body has the expected structure
                // check that all the mandatory fields are populated
                // without inspecting the field values.
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                // if parsing fails, do not match the request
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email: String = SafeEmail().fake();
        let sender = SubscriberEmail::parse(&email).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()))
            .expect("create client email failed");
        let response = ResponseTemplate::new(500).set_delay(std::time::Duration::from_secs(180));
        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .and(method("POST"))
            // use ur custom matcher
            .and(SendEmailBodyMatcher)
            .respond_with(response)
            .expect(1)
            .mount(&mock_server)
            .await;
        let email: String = SafeEmail().fake();
        let subscriber_email = SubscriberEmail::parse(&email).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();
        // Act
        let ret = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        assert!(ret.is_err())
        // Assert
    }
}
