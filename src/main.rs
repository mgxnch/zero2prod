use zero2prod::startup;

#[tokio::main]
async fn main() {
    startup::run().await;
}
