use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Form, Router, routing::get, routing::post};
use tokio::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

pub fn app() -> Router {
    // Initialise and return a router
    Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/{name}", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

/// Binds a listener on a given port.
pub async fn listener(port: u16) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(format!("127.0.0.1:{}", port)).await
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(form): Form<FormData>) -> StatusCode {
    StatusCode::OK
}
