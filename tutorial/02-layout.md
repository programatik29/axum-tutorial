# Layout

Directory structure of a common [axum] project:

```
axum-project/
├── Cargo.toml
└── src
    └── main.rs
```

`Cargo.toml` file contains dependencies. Example:

```toml
[package]
name = "axum-project"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.3"
tokio = { version = "1", features = ["full"] }
```

`src/main.rs` contains code. Example:

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Workspace

This tutorial uses a workspace for example projects. Workspace keeps projects
grouped and projects share common dependencies.

Directory structure of a workspace:

```
workspace/
├── Cargo.toml
├── hello-world
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── generate-random-number
    ├── Cargo.toml
    └── src
        └── main.rs
```

`Cargo.toml` in workspace contains members. Example:

```toml
[workspace]

members = [
    "hello-world",
    "generate-random-number",
]
```

# [Next](03-hello-world.md)

Create a hello world application.

[axum]: https://github.com/tokio-rs/axum
