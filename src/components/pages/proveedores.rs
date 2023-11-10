use leptos::*;

use crate::components::{organisms::{header::Header, footer::Footer}, molecules::forms::provider_form::ProviderForm};

#[component]
pub fn Proveedores() -> impl IntoView {
	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
            <ProviderForm/>
            <Footer/>
        </div>
	}
}