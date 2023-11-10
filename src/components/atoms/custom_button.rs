use leptos::*;

#[component]
pub fn CustomButton(
	label: &'static str
) -> impl IntoView {
	view! {
		<button type="submit" class="inline-flex items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-white bg-blue-500 rounded-lg focus:ring-4 focus:ring-primary-200 dark:focus:ring-primary-900 hover:bg-primary-800">{label}</button>
	}
}