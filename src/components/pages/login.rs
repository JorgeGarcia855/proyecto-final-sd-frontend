use leptos::{*, html::Input};
use leptos_router::*;

use crate::components::api::user::{login, Usuarios};

#[component]
pub fn Login() -> impl IntoView {
    let user_ref: NodeRef<Input> = create_node_ref();
    let pass_ref: NodeRef<Input> = create_node_ref();

    let on_submit = move |_| {
        let user = user_ref.get().expect("element not exitent").value();
        let pass = pass_ref.get().expect("element not exitent").value();

        if !user.is_empty() && !pass.is_empty() {
            let body = Usuarios {usuario: user, password: pass, ..Default::default()};
            wasm_bindgen_futures::spawn_local(async move { login(body).await });
        }

        gloo_dialogs::alert("Debes llenar los datos");
    };

	view! {
		<div class="min-h-screen bg-white dark:bg-gray-900 py-6 flex flex-col justify-center sm:py-12">
            <div class="relative py-3 sm:max-w-xl sm:mx-auto">
                <div
                    class="absolute inset-0 bg-gradient-to-r from-blue-300 to-blue-600 shadow-lg transform -skew-y-6 sm:skew-y-0 sm:-rotate-6 sm:rounded-3xl">
                </div>
                <div class="relative px-4 py-10 bg-white shadow-lg sm:rounded-3xl sm:p-20">
                    <div class="max-w-md mx-auto">
                        <div>
                            <h1 class="text-2xl font-semibold">"Bievenidos a la Tienda Generica"</h1>
                        </div>
                        <div class="divide-y divide-gray-200">
                            <Form action="">
                                <div class="py-8 text-base leading-6 space-y-4 text-gray-700 sm:text-lg sm:leading-7">
                                    <div class="relative">
                                        <input node_ref=user_ref autocomplete="off" id="user" name="user" type="text" class="peer placeholder-transparent h-10 w-full border-b-2 border-gray-300 text-gray-900 focus:outline-none focus:borer-rose-600" placeholder="Usuario" />
                                        <label for="user" class="absolute left-0 -top-3.5 text-gray-600 text-sm peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-440 peer-placeholder-shown:top-2 transition-all peer-focus:-top-3.5 peer-focus:text-gray-600 peer-focus:text-sm">"Usuario"</label>
                                    </div>
                                    <div class="relative">
                                        <input node_ref=pass_ref autocomplete="off" id="password" name="password" type="password" class="peer placeholder-transparent h-10 w-full border-b-2 border-gray-300 text-gray-900 focus:outline-none focus:borer-rose-600" placeholder="Contraseña" />
                                        <label for="password" class="absolute left-0 -top-3.5 text-gray-600 text-sm peer-placeholder-shown:text-base peer-placeholder-shown:text-gray-440 peer-placeholder-shown:top-2 transition-all peer-focus:-top-3.5 peer-focus:text-gray-600 peer-focus:text-sm">"Contraseña"</label>
                                    </div>
                                    <div class="relative">
                                        <button type="submit" on:click=on_submit class="bg-blue-500 text-white rounded-md px-2 py-1">"Iniciar Sesion"</button>
                                    </div>
                                </div>
                            </Form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
	}
}