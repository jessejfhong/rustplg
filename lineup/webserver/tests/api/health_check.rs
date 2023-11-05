use crate::helpers::TestApp;
use lineup::helpers::sayhello2;
use reqwest::Client;

#[tokio::test]
async fn health_check_should_return_200_ok() {
    let app = TestApp::new().await;

    let response = reqwest::get(format!("{}/health_check", &app.address))
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
}
