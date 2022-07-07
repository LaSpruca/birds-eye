use crate::components::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/static/index.html")]
    Index,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::NotFound => html! {<NotFound />},
        Route::Home | Route::Index => html! {<Home />},
    }
}
