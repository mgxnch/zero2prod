use axum::extract::Path;
use axum::{Router, routing::get};

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello {}", name)
}

#[tokio::main]
async fn main() {
    // Initialise a router
    let app = Router::new()
        .route("/", get(|| async { "hello world!" }))
        .route("/{name}", get(greet));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
