use reqwasm::http::Request;
use yew::prelude::*;

use crate::components::atoms::entities::Clientes;

#[function_component(ClientRow)]
fn client_row() -> Html {
	let clientes = use_state(|| None);
	let clientes_clone = clientes.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let fetch_clientes: Vec<Clientes> = Request::get("http://localhost:8080/api/clientes/")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
 
		clientes.set(Some(fetch_clientes));
    });

	
	match clientes_clone.as_ref() {
		Some(c) => html!{ 
			<>
				{c.iter().map(|client| html! { 
					<tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
						<th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
							{&client.cedula}
						</th>
						<td class="px-6 py-4">
							{&client.direccion}
						</td>
						<td class="px-6 py-4">
							{&client.email}
						</td>
						<td class="px-6 py-4">
							{&client.nombre}
						</td>
						<td class="px-6 py-4">
							{&client.telefono}
						</td>
					</tr>
				}).collect::<Vec<_>>()}
			</>	
		},
		None => html! { 
			<tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
				<th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
					{"no data"}
				</th>
				<td class="px-6 py-4">
					{"no data"}
				</td>
				<td class="px-6 py-4">
					{"no data"}
				</td>
				<td class="px-6 py-4">
					{"no data"}
				</td>
				<td class="px-6 py-4">
					{"no data"}
				</td>
			</tr>
		}
	} 
		
	
}

#[function_component(ClientTable)]
pub fn client_table() -> Html {
    html! {
		<div class="relative overflow-x-auto shadow-md sm:rounded-lg">
			<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
				<caption class="p-4 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
					{"Clientes"}
					<p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">{"Lista de clientes actuales"}</p>
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
