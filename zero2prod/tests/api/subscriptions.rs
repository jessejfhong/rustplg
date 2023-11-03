use crate::helpers::{cleanup, TestApp};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const POST: &str = "POST";
const SEND_EMAIL_END_POINT: &str = "/v2/email/outbound-emails";

const SUBSCRIBE_FORM_BODY: &str = "name=le%20guin&email=ursula_le_guin%40gmail.com";

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = TestApp::new().await;

    Mock::given(path(SEND_EMAIL_END_POINT))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(SUBSCRIBE_FORM_BODY).await;

    assert_eq!(200, response.status().as_u16());

    cleanup(&app).await;
}

#[tokio::test]
async fn subscribe_persists_the_new_subscriber() {
    let app = TestApp::new().await;

    Mock::given(path(SEND_EMAIL_END_POINT))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(SUBSCRIBE_FORM_BODY).await;

    let saved = sqlx::query!("select email, name, status from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "pending_confirmation");

    cleanup(&app).await;
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = TestApp::new().await;

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

    cleanup(&app).await;
}

#[tokio::test]
//#[should_panic]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = TestApp::new().await;

    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didn't return a 200 OK when the payload was {}",
            description
        );
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = TestApp::new().await;

    Mock::given(path(SEND_EMAIL_END_POINT))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(SUBSCRIBE_FORM_BODY).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    assert_eq!(confirmation_links.html, confirmation_links.text);
}

#[tokio::test]
async fn sbscribe_failes_if_there_is_a_fatal_database_error() {
    let app = TestApp::new().await;

    sqlx::query!("alter table subscription_tokens drop column subscription_token")
        .execute(&app.db_pool)
        .await
        .unwrap();

    let response = app.post_subscriptions(SUBSCRIBE_FORM_BODY).await;

    assert_eq!(response.status().as_u16(), 500);
}
