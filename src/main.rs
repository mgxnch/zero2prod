#[tokio::main]
async fn main() {
    let app = zero2prod::app();

    // Define listener - protocol, address and port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}
