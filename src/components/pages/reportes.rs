use crate::components::{organisms::navbar::Navbar, molecules::{client_table::ClientTable, user_table::UserTable}};
use yew::prelude::*;



#[function_component(Reportes)]
pub fn reportes() -> Html {
    
    

    html! {
        <div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Navbar />
            <ClientTable/>
            <UserTable/>   
            
           // <Footer />
        </div>
    }
}