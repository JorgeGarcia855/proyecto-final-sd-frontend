use crate::{components::organisms::navbar::Navbar, router::Route};
use yew::prelude::*;

#[function_component(Productos)]
pub fn productos() -> Html {
    html! {
        <div>
            <Navbar />
            <h1>{"productos"}</h1>
        </div>
    }
}
