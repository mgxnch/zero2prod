use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute requests.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // Port 0 will trigger an OS scan for an available port that's randomly selected
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server: actix_web::dev::Server = zero2prod::run(listener).expect("Failed to bind address");

    // tokio::spawn allows us to run our app as a background application. It takes a future
    // (which our server is), and hands it over to the runtime for polling without
    // needing to wait for its completion
    let _ = tokio::spawn(server);

    // Return the appliecation address to the caller
    format!("http://127.0.0.1:{}", port)
}
