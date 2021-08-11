# First App

## Setup

We can create a rust project using cargo tool.

```
cargo new first-app
```

Structure of a rust project looks like this:

```
first-app/
├── Cargo.toml
└── src
    └── main.rs
```

Dependencies can be included in `Cargo.toml`, format looks like:

```toml
[package]
name = "first-app"
version = "0.1.0"
authors = ["Programatik <programatik29@gmail.com>"] # Add authors
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies] # Add dependencies below
```

To create [`axum`] app, [`tokio`] as async runtime is needed. Lets add it below `[dependencies]`.

```
tokio = { version = "1.9.0", features = ["full"] }
```

[`tokio`] has many features, but those are optional and you can choose the features. For convenience `features = ["full"]` can be used.

Add [`axum`].

```
axum = "0.1.3"
```

Note: Ordering of dependencies does not matter. If you put axum above tokio, nothing will change.

Your `Cargo.toml` should look like [this](./first-app/Cargo.toml).

## Code

Time to code your first app.

First, you need to start [`tokio`] runtime. Which can be done easily like this:

```rust
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
```

Import [`axum`] modules into scope by putting the code above at the top.

```rust
use axum::prelude::*;
```

Create an [`axum`] app that responds `Hello, world!` to requests on "/" endpoint and start the server.

```rust
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
```

Your project should look like [this](./first-app/src/main.rs).

Run the code:

```
cargo run
```

Enter `http://localhost:3000/` on your browser.

Previous: [Introduction](../01-introduction)

Next: [Generate Random Number](../03-generate-random-number)

[`axum`]: https://github.com/tokio-rs/axum
[`tokio`]: https://github.com/tokio-rs/tokio
