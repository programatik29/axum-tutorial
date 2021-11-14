# Introduction

## What is Axum?

[axum] is a web application framework that focuses on ergonomics and
modularity.

### Modularity

[axum] is built on [tower] abstractions. Those abstractions:

- Are protocol agnostic. Which means you can use same code for multiple
  protocols like http and grpc.
- Has built in middlewares and utilities which you can use with [axum].
- Allow lower level access. This makes it easy to create libraries to work with
  [axum]. You can find useful libraries [here][ecosystem].

## Why use Axum?

[axum] doesn't reinvent everything. In fact [axum] is just a tool to create
[tower] abstractions that saves you writing tons of boilerplate.

Commonly, an [axum] app uses:

- [tokio] for async runtime and utilities, 
- [hyper] for http server.
- [tower] and [tower-http] for middleware and utilities.

All of those libraries are very well tested, maintained and used in production.

# [Next](02-layout.md)

Overview of [axum] project layouts.

[axum]: https://github.com/tokio-rs/axum
[ecosystem]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md
[hyper]: https://github.com/hyperium/hyper
[tokio]: https://github.com/tokio-rs/tokio
[tower]: https://github.com/tower-rs/tower
[tower-http]: https://github.com/tower-rs/tower-http
[tokio team]: https://github.com/tokio-rs
