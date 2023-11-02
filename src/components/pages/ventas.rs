use crate::{components::organisms::navbar::Navbar, router::Route};
use yew::prelude::*;

#[function_component(Ventas)]
pub fn ventas() -> Html {
    html! {
        <div>
            <Navbar />
            <h1>{"ventas"}</h1>
        </div>
    }
}
