use leptos::*;
use crate::components::{molecules::tables::rows::{LoadingRow, NoDataRow, HeaderRow}, api::sales::{Ventas, fetch_sales}};

#[component]
pub fn SalesTable() -> impl IntoView {
	let headers = vec!["Cedula","Valor Total Ventas"];
	view! {
        <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
            <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                <caption class="p-4 text-lg font-semibold text-left text-gray-900 bg-white dark:text-white dark:bg-gray-800">
                    "Ventas"
                    <p class="mt-1 text-sm font-normal text-gray-500 dark:text-gray-400">"Total de ventas por cliente"</p>
                </caption>
                <HeaderRow headers={headers}/> 
                <tbody>
                <SalesRow/>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn SalesRow() -> impl IntoView {
	let sales = create_rw_signal::<Option<Vec<Ventas>>>(None);
    let fetched_sale = create_resource(sales, |_| async move { fetch_sales().await });
    view! {
        {move || if let Some(cli) = fetched_sale.get() {
            if let Some(list) = cli {
                list.into_iter().map(move |sale| {
                    view! {
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                            <th scope="row" class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white">{sale.cedula_cliente}</th>
                            <td class="px-6 py-4">{sale.total_venta}</td>
                        </tr>
                    }
                }).collect::<Vec<_>>().into_view()
            } else { view! { <NoDataRow size=2/> } }  
        } else { view! { <LoadingRow size=1/> } }}
    }
}