use crate::socket_worker::{InMsg, OutMsg, ServerSocket};
use log::{debug, error};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{use_bridge, UseBridgeHandle};

#[function_component(Home)]
pub fn home() -> Html {
    let content = use_state(|| "".to_string());
    let input_ref = use_node_ref();

    let bridge: UseBridgeHandle<ServerSocket> = use_bridge({
        let content = content.clone();
        move |msg| match msg {
            OutMsg::Hello(msg) => content.set(msg),
        }
    });

    let send_msg = {
        let input_ref = input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            let target: HtmlInputElement = match input_ref.cast() {
                Some(elm) => elm,
                None => {
                    error!("Element was not Input");
                    return;
                }
            };

            let text = target.value();

            debug!("{text}");
            bridge.send(InMsg::Hello(text));
        })
    };

    html! {
        <div class="com-home">
            <h1>{"Hello bois!!"}</h1>

            <input ref={input_ref} />
            <button onclick={send_msg}>{"Send message"}</button>

            <p>{(*content).clone()}</p>
        </div>
    }
}
