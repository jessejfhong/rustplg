use crate::helpers::{cleanup, TestApp};
use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const POST: &str = "POST";
const SEND_EMAIL_END_POINT: &str = "/v2/email/outbound-emails";

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    let app = TestApp::new().await;

    let response = reqwest::get(&format!("{}/subscriptions/confirm", app.address))
        .await
        .unwrap();

    let addr1 = "http://127.0.0.1/subscriptions/gglo";
    let addr2 = format!("{}/subscriptions/gglo", app.address);
    dbg!(&addr2);
    let res1 = reqwest::get(addr1).await.unwrap();
    let res2 = reqwest::get(addr2).await.unwrap();

    // For adde1, it's supposed to be 404, but seems that a server is listening at port 80, and respond with ok for any get request for any path;
    assert_eq!(res1.status(), 200);
    assert_eq!(res2.status(), 404);

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    let app = TestApp::new().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path(SEND_EMAIL_END_POINT))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    let confirmation_links = app.get_confirmation_links(email_request);

    let response = reqwest::get(confirmation_links.html).await.unwrap();

    assert_eq!(response.status().as_u16(), 200);

    cleanup(&app).await;
}

#[tokio::test]
async fn clicking_on_the_confirmation_link_confirms_a_subscriber() {
    let app = TestApp::new().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path(SEND_EMAIL_END_POINT))
        .and(method(POST))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    //dbg!(&confirmation_links.html);
    reqwest::get(confirmation_links.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    let saved = sqlx::query!("select email, name, status from subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.status, "confirmed");

    cleanup(&app).await;
}
