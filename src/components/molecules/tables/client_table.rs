use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn client_table() -> impl IntoView {
	view! {
    <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
        <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
            <caption class="p-4 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
                {"Clientes"}
                <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">
                    {"Lista de clientes actuales"}
                </p>
            </caption>
            <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                <tr>
                    <th scope="col" class="px-6 py-3">
                        {"Cedula"}
                    </th>
                    <th scope="col" class="px-6 py-3">
                        {"Direccion"}
                    </th>
                    <th scope="col" class="px-6 py-3">
                        {"Email"}
                    </th>
                    <th scope="col" class="px-6 py-3">
                        {"Nombre"}
                    </th>
                    <th scope="col" class="px-6 py-3">
                        {"Telefono"}
                    </th>
                </tr>
            </thead>
            <tbody>
                <ClientRow/>
            </tbody>
        </table>
    </div>
}
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Clientes {
    cedula: i64,
    direccion: String,
    email: String,
    nombre: String,
    telefono: String,
}

async fn fetch_clientes() -> Option<Vec<Clientes>> {
	let fetch: Vec<Clientes> = reqwest::get("http://localhost:8080/api/clientes/")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
	Some(fetch)
}

#[component]
fn ClientRow() -> impl IntoView {
	let clients = create_rw_signal::<Option<Vec<Clientes>>>(None);

	wasm_bindgen_futures::spawn_local(async move {
		clients.set(fetch_clientes().await)
	});

	view! {
    {move || match clients.get() {
        Some(cli) => {
            cli.iter()
                .map(move |clien| {
                    view! {
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                            <th
                                scope="row"
                                class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
                            >
                                {clien.cedula.clone()}
                            </th>
                            <td class="px-6 py-4">{clien.direccion.clone()}</td>
                            <td class="px-6 py-4">{clien.email.clone()}</td>
                            <td class="px-6 py-4">{clien.nombre.clone()}</td>
                            <td class="px-6 py-4">{clien.telefono.clone()}</td>
                        </tr>
                    }
                })
                .collect::<Vec<_>>()
                .into_view()
        }
        None => {
            view! {
                <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                    <th
                        scope="row"
                        class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
                    >
                        {"no data"}
                    </th>
                    <td class="px-6 py-4">{"no data"}</td>
                    <td class="px-6 py-4">{"no data"}</td>
                    <td class="px-6 py-4">{"no data"}</td>
                    <td class="px-6 py-4">{"no data"}</td>
                </tr>
            }
                .into_view()
        }
    }}
}
}
