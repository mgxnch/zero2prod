use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40@gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        // If your test fails, the log reads rather naturally:
        // assertion `left == right` failed: The API did not fail with 400 Bad Request when the payload was missing the email
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
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
