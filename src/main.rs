use std::net::TcpListener;

use new_online_librarian_backend::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")
    .expect("Failed to bind address.");
    run(listener)?.await
}
