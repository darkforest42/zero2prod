use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let ln = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind");
    let ln = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind");
    zero2prod::run(ln)?.await
}
