mod components;
mod router;

use router::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
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
    yew::start_app::<App>();
}
