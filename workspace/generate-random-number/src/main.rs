use axum::{extract::Query, handler::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    // Bind to 127.0.0.1(aka localhost) address and 3000 port.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Query(range): Query<RangeParameters>) -> String {
    thread_rng().gen_range(range.start..range.end).to_string()
}
