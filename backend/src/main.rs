use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Server running on http://{}", addr);
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn index() -> String {
    format!("Hello, World!")
}
