use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // ? works on Option and Result. If it's an Option, it returns None or unwrap it
    // If it's a Result<T, E>, it returns Err(E) early if E, else it evaluates to T
    let listener = TcpListener::bind("127.0.0.1:0")?;
    run(listener)?.await
}
