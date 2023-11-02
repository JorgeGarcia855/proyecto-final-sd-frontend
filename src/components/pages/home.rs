use yew::prelude::*;

use crate::components::organisms::navbar::Navbar;

#[function_component(Home)]
pub fn home() -> Html {
    html! { <Navbar/> }
}