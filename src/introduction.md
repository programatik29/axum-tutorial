# Introduction

A complete axum web framework tutorial for version `0.6.x`.

This tutorial aims to teach how to build web applications using [axum] web
framework and assumes you know rust. If you are new to [rust], reading [the
rust programming language] book is recommended.

## What is axum?

axum is a web framework developed by [David Pedersen].

axum provides utilities to create tower services which can be served using a
compatible HTTP web server. This process will be explained later in tutorial.

## Why use axum?

With axum you can conveniently create fast, flexible and modular web
applications.

- Fast: Allows you to write really fast applications. See [this
  benchmark](https://github.com/programatik29/rust-web-benchmarks/blob/master/result/hello-world.md)
  comparing axum with other rust frameworks.
- Flexible: Provides bunch of ways you can change your applications behavior.
- Modular: You can use/reuse custom and existing utilities.
- Minimal: Doesn't aim to solve every single possible problem, instead by using
  [tower] as an interface, allows you to easily customize and implement
  features you want. This might be really important if you need a feature that
  isn't available in the ecosystem.

### Development - Todo

> - [X] Create tutorial structure.
> - [ ] Fill empty sections.
> - [ ] Support with examples.
> - [ ] Rearrange contents if needed.
> - [ ] Ask feedback from new users.

[axum]: https://github.com/tokio-rs/axum
[rust]: https://www.rust-lang.org/
[the rust programming language]: https://doc.rust-lang.org/book/
[David Pedersen]: https://github.com/davidpdrsn
[tower]: https://github.com/tower-rs/tower
[tokio team]: https://github.com/tokio-rs
