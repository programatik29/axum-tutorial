# Generate Random Number

Add project name to [workspace toml](workspace/Cargo.toml).

```toml
[workspace]

members = [
	"hello-world",
	"generate-random-number", # <--
]
```

Create the project.

```
cargo new generate-random-number
```

## Dependencies

```toml
[package]
name = "generate-random-number"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.3"
rand = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

[rand] is a random number generation library.

[serde] is a framework for serializing and deserializing Rust data structures
efficiently and generically. This library will be used to parse request query
to a struct.

## Code

```rust
use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
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

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}
```

## Run

Execute this command in workspace directory:

```
cargo run --bin generate-random-number
```

Enter `http://localhost:3000/?start=50&end=100` on your browser.

# [Next](05-include-html.md)

Include html from file in compile time.

[axum]: https://crates.io/crates/axum
[rand]: https://crates.io/crates/rand
[serde]: https://crates.io/crates/serde
