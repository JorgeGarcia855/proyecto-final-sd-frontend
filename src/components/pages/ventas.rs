use leptos::*;

use crate::components::{organisms::{header::Header, footer::Footer}, molecules::forms::sales_form::SalesForm};

#[component]
pub fn Ventas() -> impl IntoView {
	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
            <SalesForm/>
            <Footer/>
        </div>
	}
}