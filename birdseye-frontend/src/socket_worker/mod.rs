mod in_msg;
mod out_msg;

use std::collections::HashSet;

use birdseye_common::frontend::WsMessage;
use futures::channel::mpsc::{channel, Sender};
use futures::{SinkExt, StreamExt};
use gloo::net::websocket::WebSocketError;
use gloo::net::websocket::{futures::WebSocket, Message};
pub use in_msg::InMsg;
use log::{debug, error};
pub use out_msg::OutMsg;
use wasm_bindgen_futures::spawn_local;
use yew_agent::{Agent, AgentLink, HandlerId, Public};

pub struct ServerSocket {
    link: AgentLink<Self>,
    tx: Sender<Result<Message, WebSocketError>>,
    subscribers: HashSet<HandlerId>,
}

impl ServerSocket {
    fn broadcast(&self, msg: OutMsg) {
        for entry in self.subscribers.iter() {
            self.link.respond(*entry, msg.clone());
        }
    }
}

impl Agent for ServerSocket {
    type Reach = Public<Self>;
    type Message = WsMessage;
    type Input = InMsg;
    type Output = OutMsg;

    fn create(link: AgentLink<Self>) -> Self {
        let ws = WebSocket::open("wss://be.laspruca.nz:3030/dashboard")
            .expect("Could not create connection to websocket (be.laspruca.nz)");

        let (write, mut read) = ws.split();
        let (tx, rx) = channel(10);

        let socket_link = link.clone();
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Err(ex) => {
                        error!("Error receiving message from websocket {ex}");
                    }

                    Ok(msg) => match msg {
                        Message::Text(_) => {
                            error!("Should never recieve text from server, wtf?");
                        }
                        Message::Bytes(bytes) => {
                            let bytes: WsMessage = match bincode::deserialize(bytes.as_ref()) {
                                Ok(val) => val,
                                Err(ex) => {
                                    error!("Could not decode message from server {ex}");
                                    continue;
                                }
                            };

                            socket_link.send_message(bytes);
                        }
                    },
                };
            }
        });

        spawn_local(async move {
            rx.forward(write)
                .await
                .expect("Could not send message to server");
        });

        Self {
            link,
            tx,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        debug!("Got server response {msg:?}");
        match msg {
            WsMessage::Hello(msg) => self.broadcast(OutMsg::Hello(msg)),
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        let bytes = bincode::serialize(&msg).expect("Error serializing message");
        let mut tx = self.tx.clone();
        debug!("Got event: {msg:?}");

        spawn_local(async move {
            tx.send(Ok(Message::Bytes(bytes))).await.unwrap();
        });
    }

    fn name_of_resource() -> &'static str {
        "static/wasm.js"
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
