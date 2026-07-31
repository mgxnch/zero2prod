use crate::configuration::get_configuration;
use crate::routes::{app, listener};

pub async fn run() {
    // Read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // Initialise the Router and Listener
    let app = app();
    let listener = listener(configuration.application_port).await.unwrap();

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}
