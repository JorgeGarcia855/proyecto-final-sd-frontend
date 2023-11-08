use crate::components::{organisms::{navbar::Navbar, footer::Footer}, molecules::user_form::UserForm};
use yew::prelude::*;

#[function_component(Usuarios)]
pub fn usuarios() -> Html {
    html! {
        <div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Navbar/>
            <UserForm/>
            <Footer/>
        </div>
    }
}
