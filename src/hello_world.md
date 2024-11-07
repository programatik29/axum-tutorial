# Hello World

A simple hello world application looks like this:

Cargo.toml

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
# "?" is shorter than ".unwrap()"
anyhow = "1"
# full features because of laziness
tokio = { version = "1", features = ["full"] }
# import the greatest framework
axum = "0.7"
```

main.rs

```rust, no_run
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new().route("/hello-world", get(hello_world));

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}
```

If we enter "http://localhost:3000/hello-world" to browser, we can see "Hello,
world!" text coming up.

What is going on in this example?

```rust, no_run
let app = Router::new().route("/hello-world", get(hello_world));
```

This part is easy. Does the request have "/hello-world" in its URL? Okay route
it to method router (in this case "get"). Does the request have "GET" method?
Okay route it to "hello_world" *handler*.

NOTE: You can add more routes by calling `.route` again on `Router`.

Lets look at `hello_world` function which we will now refer as `hello_world` handler.

```rust, no_run
async fn hello_world() -> &'static str {
    "Hello, world!"
}
```

This function can be used as a *handler* because it is async and its return
type implements [`IntoResponse`] trait. We will look into [`IntoResponse`]
trait in detail later. For now lets use this powerful knowledge to add a
functionality to our app.

It would be cool if we could redirect all URLs coming to our application to
`hello_world` right?

By randomly traveling in [axum docs] (which is how you will operate from now
on) I found us [`Router::fallback`] method and [`Redirect`] type we can use.

```rust, no_run
use axum::{response::Redirect, routing::get, Router};
//          ^ import this ^

let app = Router::new()
    .route("/hello-world", get(hello_world))
    .fallback(anything_else); // <-- add that fallback

// create an easy handler
async fn anything_else() -> Redirect {
    Redirect::to("/hello-world")
}
```

Do these changes, run the app then go to "http://localhost:3000/plz_no_404" and
boom! It redirects us to "/hello-world".

[axum docs]: https://docs.rs/axum/0.7.7/axum/index.html
[`IntoResponse`]: https://docs.rs/axum/0.7.7/axum/response/trait.IntoResponse.html#impl-IntoResponse-for-%26str
[`Router::fallback`]: https://docs.rs/axum/0.7.7/axum/routing/struct.Router.html#method.fallback
[`Redirect`]: https://docs.rs/axum/0.7.7/axum/response/struct.Redirect.html
