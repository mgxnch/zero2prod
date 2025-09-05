#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute requests.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind to address");
    // tokio::spawn allows us to run our app as a background application. It takes a future
    // (which our server is), and hands it over to the runtime for polling without
    // needing to wait for its completion
    let _ = tokio::spawn(server);
}
