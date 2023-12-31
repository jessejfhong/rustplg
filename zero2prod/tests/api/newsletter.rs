use crate::helpers::ConfirmationLinks;
use crate::helpers::TestApp;
use reqwest::Client;
use wiremock::matchers::{any, header, method, path};
use wiremock::{Mock, ResponseTemplate};

const POST: &str = "POST";
const SEND_EMAIL_END_POINT: &str = "/v2/email/outbound-emails";
const SUBSCRIBE_FORM_BODY: &str = "name=le%20guin&email=ursula_le_guin%40gmail.com";

#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // a brand new app and empty database is setup for each integration test
    let app = TestApp::new().await;
    // crate state using public api
    create_unconfirmed_subscriber(&app).await;

    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
            "text": "Newsletter body as plain text",
            "html": "<p>Newsletter body as html</p>",
        }
    });

    let response = app.post_newsletters(newsletter_request_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    // do not use _ but _g, otherwise it will be dropped and receiving request
    let _g = Mock::given(path(SEND_EMAIL_END_POINT))
        .and(header("Content-Type", "application/json"))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .named("Create unconfirmed subscrber")
        //.expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.post_subscriptions(SUBSCRIBE_FORM_BODY)
        .await
        .error_for_status()
        .unwrap();

    let email_request = &app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();

    app.get_confirmation_links(email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_link = create_unconfirmed_subscriber(app).await;

    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    let app = TestApp::new().await;
    create_confirmed_subscriber(&app).await;

    Mock::given(path(SEND_EMAIL_END_POINT))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let newsletter_request_body = serde_json::json!({
        "title": "Newsletter title",
        "content": {
            "text": "newsletter body as plain text",
            "html": "<p>newsletters body as html</p>"
        }
    });

    let response = app.post_newsletters(newsletter_request_body).await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    let app = TestApp::new().await;

    let test_cases = vec![
        (
            serde_json::json!({
                "content": {
                    "text": "newsletter body as plain text",
                    "html": "<p>newsletters body as html</p>"
                }
            }),
            "missing title",
        ),
        (
            serde_json::json!({"title": "Newsletter!"}),
            "missing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_newsletters(invalid_body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not failed 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

#[tokio::test]
async fn requests_missing_authorization_are_rejected() {
    let app = TestApp::new().await;

    let response = Client::new()
        .post(&format!("{}/newsletters", &app.address))
        .json(&serde_json::json!({
            "title": "Newsletter title",
            "content": {
                "text": "newsletter body as plain text",
                "html": "<p>newsletters body as html</p>"
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status().as_u16());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}
