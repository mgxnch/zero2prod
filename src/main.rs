use sqlx::PgPool;
use tracing_subscriber::EnvFilter;
use zero2prod::{configuration, startup};

#[tokio::main]
async fn main() {
    // Initialise tracing subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Read configuration required to start the application
    // Panics if configuration file cannot be read
    let config = configuration::get_configuration().expect("Failed to read configuration");

    // Bind a listener to a port
    // Panics if port cannot be bound
    let listener = startup::listener(config.application_port).await.unwrap();

    // Initialise a DB connection pool
    // Panics if cannot connect to Postgres
    let pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // Start the server
    startup::run(listener, pool).await;
}
