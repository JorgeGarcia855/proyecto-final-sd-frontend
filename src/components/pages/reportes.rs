use leptos::*;

use crate::components::{organisms::{header::Header, footer::Footer}, molecules::tables::{client_table::ClientTable, user_table::UserTable}};

#[component]
pub fn Reportes() -> impl IntoView {
	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
            <ClientTable/>
            <UserTable/>   
      		<Footer/>
        </div>
    }
}