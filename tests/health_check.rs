use reqwest::Client;
use zero2prod::app;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    // HTTP client to make requests in this integration test
    let client = Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Health check only responds with HTTP 200 and a blank body
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let app = app();

    // Keep bind outside of tokio::spawn because we want to
    // ensure that the bind.await call succeeds
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    // Spawn the server as a background task
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
}
