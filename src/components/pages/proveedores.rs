use crate::{components::organisms::navbar::Navbar, router::Route};
use yew::prelude::*;

#[function_component(Proveedores)]
pub fn proveedores() -> Html {
    html! {
        <div>
            <Navbar />
            <h1>{"proveedores"}</h1>
        </div>
    }
}
