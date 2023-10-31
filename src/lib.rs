mod router;
mod components;
use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route, switch};


#[function_component(App)]
pub fn app() -> Html {
    log!("asf");
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
