use yew::prelude::*;

use crate::components::organisms::{navbar::Navbar, footer::Footer};

#[function_component(Home)]
pub fn home() -> Html {
    html! { 
        <div>
            <Navbar/>
            
            <Footer/>
        </div>
    }
}