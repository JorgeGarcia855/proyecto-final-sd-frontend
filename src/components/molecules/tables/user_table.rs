use leptos::*;
use crate::components::{molecules::tables::rows::{LoadingRow, NoDataRow, HeaderRow}, api::user::{Usuarios, fetch_users}};

#[component]
pub fn UserTable() -> impl IntoView {
	let headers = vec!["Cedula","Email","Nombre","Password","Usuario"];
    view! {
		<div class="relative overflow-x-auto shadow-md sm:rounded-lg">
			<table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
				<caption class="p-4 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
					{"Usuarios"}
					<p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">{"Lista de usuarios actuales"}</p>
				</caption>
				<HeaderRow headers={headers}/>
				<tbody>
				<UserRow/>
				</tbody>
			</table>
        </div>
	}
}

#[component]
fn UserRow() -> impl IntoView {
	let users = create_rw_signal::<Option<Vec<Usuarios>>>(None);
    let fetched_user = create_resource(users, |_| async move { fetch_users().await });
    view! {
        {move || if let Some(us) = fetched_user.get() {
            if let Some(list) = us {
                list.into_iter().map(move |user| {
                    view! {
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">{user.cedula}</th>
                            <td class="px-6 py-4">{user.email}</td>
                            <td class="px-6 py-4">{user.nombre}</td>
                            <td class="px-6 py-4">{user.password}</td>
                            <td class="px-6 py-4">{user.usuario}</td>
                        </tr>
                    }
                }).collect::<Vec<_>>().into_view()
            } else { view! { <NoDataRow size=5/> } }  
        } else { view! { <LoadingRow size=4/> } }}
    }
}