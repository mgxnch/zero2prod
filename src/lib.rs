use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn health_check() -> impl Responder {
    // Usually people call .finish(), but HttpResponseBuilder also implements the
    // Responder trait, so it's fine to leave it like this.
    // HttpResponseBuilder automatically adds a .finish() under the hood.
    HttpResponse::Ok()
}

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
