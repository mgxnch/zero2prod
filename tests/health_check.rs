use reqwest::Client;
use zero2prod::{app, listener};

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
