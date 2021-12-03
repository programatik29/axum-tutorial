# Hello World

Add project name to workspace.

```toml
[workspace]

members = [
    "hello-world", # <--
]
```

Create the project.

```
cargo new hello-world
```

## Dependencies

Example `Cargo.toml`:

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.4"
tokio = { version = "1", features = ["full"] }
```

These dependencies are required to create a bare minimum [axum] application.

[tokio] is a really popular library to work with async code. [rust] doesn't
come with a default async runtime, this leaves the choice to users. [axum] only
supports [tokio] runtime. But don't worry [tokio] has everything you can
possibly want from an async library.

## Code

```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `axum::response::IntoResponse`.

    // A closure or a function can be used as handler.

    let app = Router::new().route("/", get(handler));
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}
```

## Run

Execute this command in workspace directory:

```
cargo run --bin hello-world
```

Enter `http://localhost:3000` in browser.

# [Next](04-generate-random-number.md)

Create an application that responds with a random number from query parameters.

[axum]: https://crates.io/crates/axum
[rust]: https://www.rust-lang.org
[tokio]: https://crates.io/crates/tokio
