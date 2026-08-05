use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::routes::{health_check, subscribe};

/// State object shared with all request handlers.
pub struct AppState {
    pub pool: PgPool,
}

/// Starts the application server.
pub async fn run(listener: tokio::net::TcpListener, pool: PgPool) {
    // Initialise the Router
    let app = app(pool);

    // Serve the application using the given listener
    axum::serve(listener, app).await.unwrap();
}

/// Initialises the application router.
pub fn app(pool: PgPool) -> Router {
    let state = Arc::new(AppState { pool });

    // Initialise and return a router
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

/// Binds a listener on a given port.
pub async fn listener(port: u16) -> Result<TcpListener, std::io::Error> {
    TcpListener::bind(format!("127.0.0.1:{}", port)).await
}
