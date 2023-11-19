use leptos::*;
use crate::components::{molecules::tables::rows::{LoadingRow, NoDataRow, HeaderRow}, api::client::{Clientes, fetch_clients}};

#[component]
pub fn ClientTable() -> impl IntoView {
	let headers = vec!["Cedula","Direccion","Email","Nombre","Telefono"];
	view! {
        <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <caption class="p-4 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
                    "Clientes"
                    <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">"Lista de clientes actuales"</p>
                </caption>
                <HeaderRow headers={headers}/> 
                <tbody>
                <ClientRow/>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn ClientRow() -> impl IntoView {
	let clients = create_rw_signal::<Option<Vec<Clientes>>>(None);
    let fetched_client = create_resource(clients, |_| async move { fetch_clients().await });
    view! {
        {move || if let Some(cli) = fetched_client.get() {
            if let Some(list) = cli {
                list.into_iter().map(move |clien| {
                    view! {
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">{clien.cedula}</th>
                            <td class="px-6 py-4">{clien.direccion}</td>
                            <td class="px-6 py-4">{clien.email}</td>
                            <td class="px-6 py-4">{clien.nombre}</td>
                            <td class="px-6 py-4">{clien.telefono}</td>
                        </tr>
                    }
                }).collect::<Vec<_>>().into_view()
            } else { view! { <NoDataRow size=4/> } }  
        } else { view! { <LoadingRow size=5/> } }}
    }
}
