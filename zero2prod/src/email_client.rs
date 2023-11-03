use crate::domain::SubscriberEmail;
use aws_config::retry::RetryConfig;
use aws_sdk_sesv2 as sesv2;
use aws_sdk_sesv2::operation::send_email::SendEmailOutput;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::{config::Region, meta::PKG_VERSION, Client};

const SEND_EMAIL_END_POINT: &str = "/v2/email/outbound-emails";

pub struct EmailClient {
    client: Client,
    sender: SubscriberEmail,
}

// #[derive(serde::Serialize)]
// #[serde(rename_all = "PascalCase")]
// struct SendEmailRequest<'a> {
//     from: &'a str,
//     to: &'a str,
//     subject: &'a str,
//     html_body: &'a str,
//     text_body: &'a str,
// }

impl EmailClient {
    pub fn new(
        sender: SubscriberEmail,
        timeout: std::time::Duration,
        config: aws_config::SdkConfig,
    ) -> Self {
        let client = Client::new(&config);
        Self { client, sender }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<SendEmailOutput, sesv2::Error> {
        let dest = Destination::builder()
            .to_addresses(self.sender.as_ref())
            .build();
        let subject = Content::builder().data(subject).charset("UTF-8").build();
        let html_body = Content::builder()
            .data(html_content)
            .charset("UTF-8")
            .build();
        let text_body = Content::builder()
            .data(text_content)
            .charset("UTF-8")
            .build();
        let body = Body::builder().html(html_body).text(text_body).build();
        let msg = Message::builder().subject(subject).body(body).build();
        let email = EmailContent::builder().simple(msg).build();

        self.client
            .send_email()
            .from_email_address(recipient.as_ref())
            .destination(dest)
            .content(email)
            .send()
            .await
            .map_err(sesv2::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::SubscriberEmail;
    use aws_config::timeout::TimeoutConfig;
    use claim::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use std::time::Duration;
    use wiremock::matchers::{any, header, method, path};
    use wiremock::Request;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(conf: aws_config::SdkConfig) -> EmailClient {
        EmailClient::new(email(), std::time::Duration::from_millis(200), conf)
    }

    // for validating the json body in the request
    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        // Setup
        let mock_server = MockServer::start().await;
        let conf = aws_config::from_env()
            .endpoint_url(mock_server.uri())
            .load()
            .await;
        let email_client = email_client(conf);

        Mock::given(path(SEND_EMAIL_END_POINT))
            .and(header("Content-Type", "application/json"))
            .and(method("POST"))
            //.and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let content: String = content();

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content, &content)
            .await;

        // Assert
        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        let mock_server = MockServer::start().await;
        let conf = aws_config::from_env()
            .endpoint_url(mock_server.uri())
            .retry_config(RetryConfig::standard().with_max_attempts(1))
            .load()
            .await;
        let email_client = email_client(conf);

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        let content = content();
        let outcome = email_client
            .send_email(&email(), &subject(), &content, &content)
            .await;

        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_times_out_if_the_server_takes_too_long() {
        let mock_server = MockServer::start().await;
        let conf = aws_config::from_env()
            .endpoint_url(mock_server.uri())
            .timeout_config(
                TimeoutConfig::builder()
                    .operation_timeout(Duration::from_secs(2))
                    .build(),
            )
            .load()
            .await;
        let email_client = email_client(conf);

        let reponse = ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(180));
        Mock::given(any())
            .respond_with(reponse)
            .expect(1)
            .mount(&mock_server)
            .await;

        let content = content();
        let outcome = email_client
            .send_email(&email(), &subject(), &content, &content)
            .await;

        assert_err!(outcome);
    }
}
