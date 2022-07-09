mod asset_compiler;

use crate::asset_compiler::asset_compiler;
use futures_util::{SinkExt, StreamExt};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::RwLock;
use tracing::info;
use warp::cors::Cors;
use warp::{filters::ws::Message, Filter};

fn cors() -> Cors {
    warp::filters::cors::cors().allow_any_origin().build()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::fmt().init();

    let (mut watcher, rx) = async_watcher().unwrap();
    watcher
        .configure(Config::OngoingEvents(Some(Duration::from_secs(1))))
        .unwrap();
    watcher
        .watch(".".as_ref(), RecursiveMode::Recursive)
        .expect("Could not watch directory");

    let update_fe = Arc::new(RwLock::new(false));
    let update_fe2 = update_fe.clone();

    let live_reload = warp::path("live-reload")
        .and(warp::get())
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let update_fe = update_fe2.clone();
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| async move {
                let update_fe = update_fe.clone();

                // Just echo all messages back...
                let (mut tx, _) = websocket.split();

                loop {
                    let read = update_fe.read().await;
                    if *read {
                        tx.send(Message::text("refresh"))
                            .await
                            .expect("Could not send refresh to frontend");
                        drop(read);
                        *update_fe.write().await = false;
                        break;
                    }
                }
            })
        })
        .with(cors());

    let static_files = warp::get()
        .and(warp::filters::fs::dir("birdseye-runner/pkg"))
        .with(cors());

    let server = warp::serve(live_reload.or(static_files))
        .tls()
        .key_path("cert/key.pem")
        .cert_path("cert/cert.pem")
        .run(([127, 0, 0, 1], 42069));

    let watcher = asset_compiler(update_fe, rx);

    tokio::select! {
        _ = server => {},
        _ = watcher => {}
    };
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res| {
        tx.blocking_send(res).expect("Could not send notify event");
    })?;

    Ok((watcher, rx))
}
