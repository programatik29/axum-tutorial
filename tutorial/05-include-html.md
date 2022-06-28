# Include Html

## Dependencies

```toml
[dependencies]
axum = "0.5"
tokio = { version = "1", features = ["full"] }
```

## Code

```rust
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    Html(include_str!("../index.html"))
}
```

## Run

Run the program then enter `http://localhost:3000` on your browser.

## Next

To be continued.
