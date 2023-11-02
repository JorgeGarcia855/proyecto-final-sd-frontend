use crate::{components::organisms::navbar::Navbar, router::Route};
use yew::prelude::*;

#[function_component(Clientes)]
pub fn clientes() -> Html {
    html! {
        <div>
            <Navbar />
            <h1>{"cliente"}</h1>
        </div>
    }
}
