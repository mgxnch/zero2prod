mod health_check;
mod subscriptions;

use axum::{Router, routing::get, routing::post};
use tokio::net::TcpListener;

pub use health_check::*;
pub use subscriptions::*;

pub fn app() -> Router {
    // Initialise and return a router
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

/// Binds a listener on a given port.
pub async fn listener(port: u16) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(format!("127.0.0.1:{}", port)).await
}
