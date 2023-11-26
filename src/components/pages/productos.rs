use leptos::*;

use crate::components::organisms::{header::Header, footer::Footer};

#[component]
pub fn Productos() -> impl IntoView {
    
    let on_submit = move |_| {
        gloo_dialogs::alert("Los productos se subieron")
    };

	view! {
		<div class="bg-white dark:bg-gray-900 flex flex-col min-h-screen">
            <Header/>
                <form class="max-w-lg mx-auto" on:submit=on_submit target="dummyframe" action="http://127.0.0.1:8080/api/productos/" method="post" enctype="multipart/form-data">
                    <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" for="user_avatar">"Subir Archivo"</label>
                    <input class="block w-full text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer bg-gray-50 dark:text-gray-400 focus:outline-none dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400" aria-describedby="user_avatar_help" accept=".csv" id="user_avatar" name="productos" type="file"/>
                    <div class="mt-1 text-sm text-gray-500 dark:text-gray-300" id="user_avatar_help">"Sube los productos nuevos aqui (Solo CSV)"</div>
                    <input class="inline-flex items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-white bg-blue-500 rounded-lg focus:ring-4 focus:ring-primary-200 dark:focus:ring-primary-900 hover:bg-primary-800" type="submit" value="Upload"/>
                </form> 
                <iframe name="dummyframe" id="dummyframe" style="display: none;"></iframe>
            <Footer/>
        </div>
	}
}