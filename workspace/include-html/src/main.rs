use std::net::SocketAddr;
use axum::prelude::*;

#[tokio::main]
async fn main() {
    let app = route("/", get(handler));

    // Bind to 127.0.0.1(aka localhost) address and 3000 port.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> response::Html<&'static str> {
    response::Html(include_str!("../html/index.html"))
}
