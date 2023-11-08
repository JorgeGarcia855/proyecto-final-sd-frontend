use reqwasm::http::Request;
use yew::prelude::*;

use crate::components::atoms::entities::Usuarios;

#[function_component(UserRow)]
fn user_row() -> Html {
	let usuarios = use_state(|| None);
	let usuarios_clone = usuarios.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let fetch_usuarios: Vec<Usuarios> = Request::get("http://localhost:8080/api/usuarios/")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
 
		usuarios.set(Some(fetch_usuarios));
    });

	html! {
		{
			match usuarios_clone.as_ref() {
				Some(c) => html!{ 
					<>
						{c.iter().map(|user| html! { 
							<tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
								<th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">
									{&user.cedula}
								</th>
								<td class="px-6 py-4">
									{&user.email}
								</td>
								<td class="px-6 py-4">
									{&user.nombre}
								</td>
								<td class="px-6 py-4">
									{&user.password}
								</td>
								<td class="px-6 py-4">
									{&user.usuario}
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
	}
}

#[function_component(UserTable)]
pub fn client_table() -> Html {
    html! {
		<div class="relative overflow-x-auto shadow-md sm:rounded-lg">
			<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
				<caption class="p-4 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
					{"Usuarios"}
					<p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">{"Lista de usuarios actuales"}</p>
				</caption>
				<thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
					<tr>
						<th scope="col" class="px-6 py-3">
							{"Cedula"}
						</th>
						<th scope="col" class="px-6 py-3">
							{"Email"}
						</th>
						<th scope="col" class="px-6 py-3">
							{"Nombre"}
						</th>
						<th scope="col" class="px-6 py-3">
							{"Password"}
						</th>
						<th scope="col" class="px-6 py-3">
							{"Usuario"}
						</th>
					</tr>
				</thead>
				<tbody>
				<UserRow/>
				</tbody>
			</table>
        </div>
	}
}
