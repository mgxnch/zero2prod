use reqwest::Client;

use zero2prod::routes::{app, listener};

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app().await;

    // HTTP client to make requests in this integration test
    let client = Client::new();

    // Send request to health_check endpoint
    let url = format!("{}/health_check", addr);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request.");

    // Health check only responds with HTTP 200 and a blank body
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let addr = spawn_app().await;
    let client = Client::new();

    let body = "name=user%20one&email=foo_bar%40baz.com";
    let response = client
        .post(format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let addr = spawn_app().await;
    let client = Client::new();
    let test_cases = vec![
        ("name=foo", "missing the email"),
        ("email=foo%40bar.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Content when the payload was {}.", // additional customised error message on test failure
            error_message
        )
    }
}

/// Spawns the zero2prod server in the background on a random port. Returns the
/// application address e.g. "127.0.0.1:{port}"
async fn spawn_app() -> String {
    let app = app();

    // Keep bind outside of tokio::spawn because we want to
    // ensure that the bind.await call succeeds
    let listener = listener(0).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    // Spawn the server as a background task
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    // Return the application address for the callers (test cases) to use
    format!("http://127.0.0.1:{}", port)
}
