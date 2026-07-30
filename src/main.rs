use zero2prod::configuration::get_configuration;
use zero2prod::routes::{app, listener};

#[tokio::main]
async fn main() {
    // Read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // Initialise the Router and Listener
    let app = app();
    let listener = listener(configuration.application_port).await.unwrap();

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}
