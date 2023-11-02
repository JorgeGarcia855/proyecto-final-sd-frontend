use crate::{components::organisms::navbar::Navbar, router::Route};
use yew::prelude::*;

#[function_component(Usuarios)]
pub fn usuarios() -> Html {
    html! {
        <div>
            <Navbar />
            <h1>{"usuarios"}</h1>
        </div>
    }
}
