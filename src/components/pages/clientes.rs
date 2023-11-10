use leptos::*;

use crate::components::{organisms::{header::Header, footer::Footer}, molecules::forms::client_form::ClientForm};

#[component]
pub fn Clientes() -> impl IntoView {
	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
            <ClientForm/>
            <Footer/>
        </div>
	}
}