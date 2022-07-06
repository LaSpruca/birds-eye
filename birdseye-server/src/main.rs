mod config;

use std::{convert::Infallible, fs::read_to_string};

use crate::config::load_config;
use futures_util::{FutureExt, StreamExt};
use listenfd::ListenFd;
use warp::{
    hyper::{self, Server},
    Filter,
};

#[tokio::main]
#[cfg(debug_assertions)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter("debug,rustls=info")
        .init();

    let _config = load_config();

    let ws_route = warp::get()
        .and(warp::path("echo"))
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let files = warp::path("static").and(warp::fs::dir("birdseye-frontend/static"));
    let read = warp::get().and(warp::fs::file("birdseye-frontend/resources/index.html"));

    let routes = ws_route.or(files).or(read);

    let svc = warp::service(routes.clone());

    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();

    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&([127, 0, 0, 1], 3030).into())
    };

    server.serve(make_svc).await.unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
