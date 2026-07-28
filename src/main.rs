use zero2prod::routes::{app, listener};

#[tokio::main]
async fn main() {
    // Initialise the Router and Listener
    let app = app();
    let listener = listener(8000).await.unwrap();

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}
