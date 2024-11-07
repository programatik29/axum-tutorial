# Extractors

Axum provides extractors to get/extract data about requests to our handlers. To
qualify as a handler, parameters of a function must implement
[`FromRequestParts`] last parameter can implement [`FromRequest`] (meaning it
also implements [`FromRequestParts`]). There are two traits because some parts
of request for example body can only be extracted once (can be cloned later).

In previous hello world example we didn't need any info about requests for our
app to run. I was thinking, "Okay, I need to build some random app to explain
readers how extractors work." What did I do? Of course I found something by
randomly traveling in [axum docs].

Every developer needs a way to get their clients ip address and sell it to big
corporations. But of course as ethical developers, we will not do that. Instead
we will explain the client we got their ip address and they can possibly be
identified by harmful parties and to prevent that they need to buy our VPN
service.

Here is a basic example of our app:

```rust, no_run
use axum::{extract::ConnectInfo, routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(index));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!(
        "Your ip address is \"{addr}\".\n\
         You are in immediate danger of getting identified by bad people.\n\
         Thankfully we have a VPN service to hide your ip. \n\
         Visit this link to download it \"http://localhost:3000/average_joe_absolutely_needs_vpn\""
    )
}
```

To be able to get ip address with `ConnectInfo` extractor we need this
boilerplate code:

```rust, no_run
// copypasta for future projects
axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>(),
)
.await?;
```

Lets take a look at function signature:

```rust, no_run
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String
```

You might not be familiar of pattern matching done for function parameter
"addr" here. Lets explain shortly.

`ConnectInfo` is a struct. It is defined something like `struct
ConnectInfo<T>(T);`. It has paranthesis instead of braces and can take any type
like a tuple with single item. (example `let something = ConnectInfo(String);`)
By using tuple structs in function parameters you can match inner item. If we
used `addr: ConnectInfo<SocketAddr>` instead we would have to do `let addr =
addr.0;` in the code below to achieve the same thing.

To simplify, for now lets talk about it using the other syntax.

```rust, no_run
async fn index(addr: ConnectInfo<SocketAddr>) -> String
```

`addr` parameter type needs to implement [`FromRequest`] or
[`FromRequestParts`] for this function to qualify as a handler. [`ConnectInfo`]
implements it if we use it like how it is documented.

We are dynamically creating a string to add ip address inside. So we need to
use `String` type which implements [`IntoResponse`].

In the handler code, its basically just `format!` macro to create a string so
nothing new there.

Our app works fine but its just plaintext. Lets improve it by doing some html.

```rust, no_run
// change return type to `Html<String>` to let browser know we are sending html
async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        "<h1>Your ip address is: \"{addr}\"</h1>\n\
         <h2>You are in immediate danger of getting identified by bad people.</h2>\n\
         <h2>Thankfully we have a VPN service to hide your ip.</h2>\n\
         <h2>Visit <a href=\"http://localhost:3000/average_joe_absolutely_needs_vpn\">THIS</a> link to download it.</h2>"
    );

    // create `Html` type like this
    Html(html)
}
```

### Challenge

Create a `/average_joe_absolutely_needs_vpn` page returning html to convince
the ~~poor guy~~ client to buy our VPN service.

[axum docs]: https://docs.rs/axum/0.7.7/axum/index.html
[`FromRequest`]: https://docs.rs/axum/0.7.7/axum/extract/trait.FromRequest.html
[`FromRequestParts`]: https://docs.rs/axum/0.7.7/axum/extract/trait.FromRequestParts.html
[`ConnectInfo`]: https://docs.rs/axum/0.7.7/axum/extract/struct.ConnectInfo.html#impl-FromRequestParts%3CS%3E-for-ConnectInfo%3CT%3E
[`IntoResponse`]: https://docs.rs/axum/0.7.7/axum/response/trait.IntoResponse.html#impl-IntoResponse-for-String
