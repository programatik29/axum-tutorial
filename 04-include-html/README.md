# Include Html

## Dependencies

```toml
[dependencies]
tokio = { version = "1.9.0", features = ["full"] }
axum = "0.1.3"
```

## Code

We can use [`include_str`] macro to include a `UTF-8` file as `&'static str`. To set `text/html` content-type on request easily, [`Html`] can be used.

```rust
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
```

Previous: [Generate Random Number](../03-generate-random-number)

Next: *To be continued*

[`include_str`]: https://doc.rust-lang.org/stable/std/macro.include_str.html
[`Html`]: https://docs.rs/axum/0.1.3/axum/response/struct.Html.html
