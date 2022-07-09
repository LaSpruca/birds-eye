mod config;

use crate::config::load_config;
use futures_util::{FutureExt, StreamExt};
use warp::Filter;

#[tokio::main]
#[cfg(debug_assertions)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::net::SocketAddr;

    use tracing::{error, info};

    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug,h2=info")
        .init();

    let config = load_config();

    let ws_route = warp::get()
        .and(warp::path("dashboard"))
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        error!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let files = warp::path("static")
        .and(warp::fs::dir(config.be_server.static_path.clone()))
        .with(warp::log("Static Files"))
        .with(warp::compression::gzip());

    let mut index_file = config.be_server.static_path.clone();
    index_file.push("index.html");

    let front_end = warp::get()
        .and(warp::fs::file(index_file))
        .with(warp::log("front-end"));

    let routes = ws_route
        .with(warp::log("Frontend Webscoket"))
        .or(files)
        .or(front_end);

    let server_addr = format!("{}:{}", &config.be_server.host, config.be_server.port);
    let server_addr: SocketAddr = server_addr.parse().unwrap();

    warp::serve(routes)
        .tls()
        .key_path(config.be_server.key)
        .cert_path(config.be_server.cert)
        .run(server_addr)
        .await;

    info!("Server is vaish");

    Ok(())
}
