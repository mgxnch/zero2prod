use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn app() -> Router {
    // Initialise and return a router
    Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check))
}

/// Binds a listener on a given port.
pub async fn listener(port: u16) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(format!("127.0.0.1:{}", port)).await
}
