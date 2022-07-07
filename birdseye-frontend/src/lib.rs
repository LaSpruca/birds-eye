mod components;
mod router;
pub mod socket_worker;

use js_sys::{global, Reflect};
use router::*;
use wasm_bindgen::prelude::*;
use wasm_logger::Config;
use yew::prelude::*;
use yew_agent::Threaded;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header>
                <h1>{"BirdsEye"}</h1>
            </header>

            <main>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </main>
        </>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(Config::default());

    if Reflect::has(&global(), &JsValue::from_str("window")).unwrap() {
        yew::start_app::<App>();
    } else {
        socket_worker::ServerSocket::register();
    }
}
