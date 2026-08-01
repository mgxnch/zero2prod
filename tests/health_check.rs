use reqwest::Client;

use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::configuration::{self, Settings};
use zero2prod::startup;

#[tokio::test]
async fn health_check_works() {
    // Set up the test case
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let addr = spawn_app(&config).await;

    // Test connection to Postgres works
    let connection_string = config.database.connection_string();
    let _ = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");
    let client = Client::new();

    // Send request to health_check endpoint
    let url = format!("{}/health_check", addr);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    // Health check only responds with HTTP 200 and a blank body
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Set up the test case
    let config = configuration::get_configuration().expect("Failed to read configuration");
    let addr = spawn_app(&config).await;

    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = Client::new();

    // Send the subscribe request
    let body = "name=user%20one&email=foo_bar%40baz.com";
    let response = client
        .post(format!("{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    // Check that it was persisted to database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "user");
    assert_eq!(saved.email, "foo_bar@baz.com");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let config = configuration::get_configuration().unwrap();
    let addr = spawn_app(&config).await;
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
            .expect("Failed to execute request");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Unprocessable Content when the payload was {}.", // additional customised error message on test failure
            error_message
        )
    }
}

/// Spawns the zero2prod server in the background on a random port. Returns the
/// application URL e.g. "http://127.0.0.1:{port}"
async fn spawn_app(config: &Settings) -> String {
    // Keep listener binding outside of tokio::spawn because we want to ensure that the bind.await call succeeds first
    let listener = startup::listener(0).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // let app = startup::app(pool);

    // Spawn the server as a background task
    tokio::spawn(async move { startup::run(listener, pool).await });

    // Return the application address for the callers (test cases) to use
    format!("http://127.0.0.1:{}", port)
}
