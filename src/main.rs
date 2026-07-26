use zero2prod::{app, listener};

#[tokio::main]
async fn main() {
    // Initialise the Router and Listener
    let app = app();
    let listener = listener().await.unwrap();

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}
