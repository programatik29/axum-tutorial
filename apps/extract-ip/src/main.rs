use axum::{extract::ConnectInfo, response::Html, routing::get, Router};
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

async fn index(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        "<h1>Your ip address is: \"{addr}\"</h1>\n\
         <h2>You are in immediate danger of getting identified by bad people.</h2>\n\
         <h2>Thankfully we have a VPN service to hide your ip.</h2>\n\
         <h2>Visit <a href=\"http://localhost:3000/average_joe_absolutely_needs_vpn\">THIS</a> link to download it.</h2>"
    );

    Html(html)
}
