use leptos::*;

use crate::components::{organisms::header::Header, molecules::tables::{client_table::ClientTable, user_table::UserTable, sales_table::SalesTable}};

#[component]
pub fn Reportes() -> impl IntoView {
	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
            <ClientTable/>
            <UserTable/> 
            <SalesTable/>  
      		// <Footer/>
        </div>
    }
}