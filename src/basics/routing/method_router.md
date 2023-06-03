# Method Router

Method routers call a handler based on request method.

### Example

```rust,no_run
# extern crate axum;
# extern crate tokio;
#
use axum::routing::get;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = get(handler);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "GET method handler."
}
```

Only requests with `GET` method will go to `handler`.

### Example

Method routers can be chained together. [`MethodRouter`] documentation page
shows methods available.

```rust,no_run
# extern crate axum;
# extern crate tokio;
#
# use std::net::SocketAddr;
use axum::routing::get;

# #[tokio::main]
# async fn main() {
#     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
# 
let app = get(get_handler).post(post_handler);
# 
#     axum::Server::bind(&addr)
#         .serve(app.into_make_service())
#         .await
#         .unwrap();
# }
# 
# async fn get_handler() {}
# async fn post_handler() {}
```

`axum::routing::get` function creates a [`MethodRouter`] then the `post` method
on it is called to chain another method router.

Requests with `GET` method will go to `get_handler` and requests with `POST`
method will go to `post_handler`.

[`MethodRouter`]: https://docs.rs/axum/0.6.x/axum/routing/method_routing/struct.MethodRouter.html
