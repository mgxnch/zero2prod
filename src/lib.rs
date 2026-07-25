use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Router, routing::get};

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn run() {
    // Initialise a router
    let app = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check));

    // Define listener - protocol, address and port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    // Serve the application
    axum::serve(listener, app).await.unwrap();
}
