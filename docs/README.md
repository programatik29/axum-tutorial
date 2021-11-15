# Axum

Axum is a web application framework for writing HTTP applications with Rust and focuses on ergonomics and modularity.

### Modularity

[axum] is built on [tower] abstractions. Those abstractions:

- Are protocol agnostic. Which means you can use same code for multiple protocols like HTTP and gRPC.
- Have built-in middleware and utilities which you can use with [axum].
- Allow lower level access. This makes it easy to create libraries to work with
  [axum]. You can find useful libraries [here][ecosystem].

## Why use Axum?

[axum] doesn't reinvent everything.

Commonly, an [axum] app uses:

- [hyper] for HTTP server.
- [tokio] for async runtime and utilities,
- [tower] and [tower-http] for middleware and utilities.

All of those libraries are very well tested, maintained and used in production.

[axum]: https://github.com/tokio-rs/axum
[ecosystem]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md
[hyper]: https://github.com/hyperium/hyper
[tokio]: https://github.com/tokio-rs/tokio
[tower]: https://github.com/tower-rs/tower
[tower-http]: https://github.com/tower-rs/tower-http
[tokio team]: https://github.com/tokio-rs
