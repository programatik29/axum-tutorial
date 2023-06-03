# Handlers

Building block of an axum applications are handlers. Handlers are async
functions that implement [`Handler`] trait. If you scroll to the "implementors"
section of documentation page you will see a monstrosity. For now just call it
magic and move on.

A handler looks like this:

```rust,no_run
async fn handler(/* extractor part */) -> () /* response part */ {
    // code part
}
```

For an async rust function to be qualified as a handler:

- Parameters must implement [`FromRequestParts`] and last parameter must
  implement [`FromRequest`].
- Return type must implement [`IntoResponse`].
- Function must be [`Send`]. (don't worry about this unless you use something
  `!Send` which is uncommon)

Note: Implementations on Foreign Types section in [`FromRequestParts`],
[`FromRequest`] and [`IntoResponse`] documentation page contains some types
that can be used.

### Example

A simple handler that calculates 2 + 2.

```rust,no_run
async fn handler() -> String {
    format!("2 + 2 = {}", 2 + 2)
}
```

[`Handler`]: https://docs.rs/axum/0.6.x/axum/handler/trait.Handler.html
[`FromRequestParts`]: https://docs.rs/axum/0.6.x/axum/extract/trait.FromRequestParts.html
[`FromRequest`]: https://docs.rs/axum/0.6.x/axum/extract/trait.FromRequest.html
[`IntoResponse`]: https://docs.rs/axum/0.6.x/axum/response/trait.IntoResponse.html
[`Send`]: https://doc.rust-lang.org/stable/std/marker/trait.Send.html
