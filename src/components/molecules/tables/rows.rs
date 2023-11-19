
use leptos::{*, svg::view};

#[component]
pub fn LoadingRow(size: i16) -> impl IntoView  {
    view! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <th
                scope="row"
                class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap dark:text-white"
            >
                {"loading data"}
            </th>
            <For each=move || 0..size key=move |_| size children=move |_| view! { <td class="px-6 py-4">{""}</td> }/>
        </tr>
    }.into_view()
}

#[component]
pub fn NoDataRow(size: i16) -> impl IntoView {
    view! {
        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
            <For each=move || 0..size key=move |_| size children=move |_| view! { <td class="px-6 py-4">{"no data"}</td> }/>
        </tr>
    }.into_view()
}

#[component]
pub fn HeaderRow(headers: Vec<&'static str>) -> impl IntoView {
    view! {
        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
            <tr>
                {headers.into_iter().map(move |header| {
                    view! {
                        <th scope="col" class="px-6 py-3">
                            {header}
                        </th>
                    }
                }).collect::<Vec<_>>()}
            </tr>
        </thead>
    }
}