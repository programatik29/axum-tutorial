# Server

Creating a web application with axum requires a server library. Without a
server library, tower services created by axum are mostly useful for testing
purposes.

To run an axum application, you need [tokio] and obviously axum as dependency.
Example:

```toml
[dependencies]
axum = "0.6"
tokio = { version = "1", features = ["full"] }
```

At the time of writing, there are two popular server libraries that can be used
with axum. Those libraries are [hyper] and [axum-server]. Latter is usually
used to support TLS.

Examples of a bare minimum axum web application for both of these servers are
shown below.

### Example - hyper

```rust,no_run
# extern crate axum;
# extern crate tokio;
#
use axum::handler::HandlerWithoutStateExt;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // `axum::Server` is a re-export of `hyper::server::Server`
    axum::Server::bind(&addr)
        .serve(handler.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}
```

### Example - axum-server

```rust,no_run
# extern crate axum;
# extern crate axum_server;
# extern crate tokio;
#
use axum::handler::HandlerWithoutStateExt;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum_server::bind(addr)
        .serve(handler.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}
```

### Challenge

Run an axum application and visit it on your browser.

[tokio]: https://github.com/tokio-rs/tokio
[hyper]: https://github.com/hyperium/hyper
[axum-server]: https://github.com/programatik29/axum-server
