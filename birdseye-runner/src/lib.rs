#[cfg(target_arch = "wasm32")]
pub mod live_reload {
    use futures::StreamExt;
    use gloo_console::info;
    use gloo_net::websocket::{futures::WebSocket, Message};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::spawn_local;
    use web_sys::window;

    #[wasm_bindgen(start)]
    pub fn start() {
        let ws = WebSocket::open("wss://be.laspruca.nz:42069/live-reload").unwrap();
        info!("Connected to Live Reload server");

        let (_, mut rx) = ws.split();

        spawn_local(async move {
            while let Some(Ok(Message::Text(msg))) = rx.next().await {
                if &msg == "refresh" {
                    let window = window().unwrap();
                    window.location().reload().unwrap();
                    break;
                }
            }
            info!("Exited loop");
        });
    }
}
