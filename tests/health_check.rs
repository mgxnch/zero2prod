use std::time::Duration;

use reqwest::Client;
use tokio::time::sleep;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Spawn server as a background task, or else .await
    // will suspend our test code indefinitely, because
    // `run` does not complete. It's always running to listen
    // for new incoming connections.
    spawn_app().await;

    // Flaky test
    sleep(Duration::from_millis(100)).await;

    // HTTP client to make requests in this integration test
    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    tokio::spawn(run());
}
