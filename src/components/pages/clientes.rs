use crate::components::{
    molecules::client_form::ClientForm,
    organisms::{footer::Footer, navbar::Navbar},
};
use yew::prelude::*;

#[function_component(Clientes)]
pub fn clientes() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Navbar/>
            <ClientForm/>
            <Footer/>
        </div>
    }
}
