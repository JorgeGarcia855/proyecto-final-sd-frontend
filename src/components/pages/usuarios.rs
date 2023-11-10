use leptos::*;

use crate::components::{organisms::{header::Header, footer::Footer}, molecules::forms::user_form::UserForm};

#[component]
pub fn Usuarios() -> impl IntoView {
	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
            <UserForm/>
            <Footer/>
        </div>
	}
}