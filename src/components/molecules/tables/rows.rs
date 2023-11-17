
use leptos::*;

#[component]
pub fn LoadingRow() -> impl IntoView  {
    view! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th
                scope="row"
                class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
            >
                {"loading data"}
            </th>
            <td class="px-6 py-4">{""}</td>
            <td class="px-6 py-4">{""}</td>
            <td class="px-6 py-4">{""}</td>
            <td class="px-6 py-4">{""}</td>
        </tr>
    }.into_view()
}

#[component]
pub fn NoDataRow() -> impl IntoView {
    view! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th
                scope="row"
                class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
            >
                {"loading data"}
            </th>
            <td class="px-6 py-4">{"no data"}</td>
            <td class="px-6 py-4">{"no data"}</td>
            <td class="px-6 py-4">{"no data"}</td>
            <td class="px-6 py-4">{"no data"}</td>
        </tr>
    }.into_view()
}