use reqwest::Client;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;

pub struct TestApp {
    pub address: String, // URL of the application e.g. http://127.0.0.1:8000
    pub pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    // Set up the test case
    let test_app = spawn_app().await;

    // Send request to health_check endpoint
    let client = Client::new();
    let url = format!("{}/health_check", test_app.address);
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
    let test_app = spawn_app().await;

    // Send the subscribe request
    let client = Client::new();
    let body = "name=user%20one&email=foo_bar%40baz.com";
    let response = client
        .post(format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    // Check that it was persisted to database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&test_app.pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "user");
    assert_eq!(saved.email, "foo_bar@baz.com");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Set up the test cases
    let test_app = spawn_app().await;
    let client = Client::new();
    let test_cases = vec![
        ("name=foo", "missing the email"),
        ("email=foo%40bar.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
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
async fn spawn_app() -> TestApp {
    // Keep listener binding outside of tokio::spawn because we want to ensure that the bind.await call succeeds first
    let listener = startup::listener(0)
        .await
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let config = get_configuration().expect("Failed to read configuration");
    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Clone pool before moving it into the background task
    let db_pool = pool.clone();
    tokio::spawn(async move { startup::run(listener, db_pool).await });

    // Return TestApp struct
    TestApp {
        address,
        pool: pool.clone(),
    }
}
