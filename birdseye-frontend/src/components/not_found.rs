use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let route = use_location().unwrap().pathname();
    html! {
        <div class="error">
            <div class="bubble">
                <h1>{format!("Page '{route}' not found")}</h1>
                <Link<Route> to={Route::Home}>{"Go home?"}</Link<Route>>
            </div>
        </div>
    }
}
