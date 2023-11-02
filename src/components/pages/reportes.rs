use crate::{components::organisms::navbar::Navbar, router::Route};
use yew::prelude::*;

#[function_component(Reportes)]
pub fn reportes() -> Html {
    html! {
        <div>
            <Navbar />
            <h1>{"reportes"}</h1>
        </div>
    }
}
