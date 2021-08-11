use std::net::SocketAddr;
use axum::prelude::*;

#[tokio::main]
async fn main() {
    let app = route("/", get(|| async { "Hello, axum!" }));

    // Bind to 127.0.0.1(aka localhost) address and 3000 port.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
