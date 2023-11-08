use crate::components::{organisms::{navbar::Navbar, footer::Footer}, molecules::provider_form::ProviderForm};
use yew::prelude::*;

#[function_component(Proveedores)]
pub fn proveedores() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Navbar/>
            <ProviderForm/>
            <Footer/>
        </div>
    }
}
