use leptos::*;
use serde::{Deserialize, Serialize};
use crate::components::molecules::tables::rows::{LoadingRow, NoDataRow};

#[component]
pub fn UserTable() -> impl IntoView {
    view! {
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

#[component]
fn UserRow() -> impl IntoView {
	view! {
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