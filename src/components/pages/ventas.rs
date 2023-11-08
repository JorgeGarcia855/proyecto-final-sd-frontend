use crate::components::{organisms::{navbar::Navbar, footer::Footer}, molecules::sales_form::SalesForm};
use yew::prelude::*;

#[function_component(Ventas)]
pub fn ventas() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Navbar/>
            <SalesForm/>
            <Footer/>
        </div>
    }
}
