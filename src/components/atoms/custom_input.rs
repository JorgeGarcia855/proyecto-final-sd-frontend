use leptos::{*, html::Input};

#[component]
pub fn CustomInput(
	name: &'static str,
    input_type: &'static str,
    id: &'static str,
	#[prop(optional)]
    placeholder: &'static str,
	#[prop(optional)]
	node_ref: NodeRef<Input>,
	#[prop(default = false)]
    required: bool,
) 
-> impl IntoView {
	view! {
		<div class="sm:col-span-2">
			<label for={id} class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{name}</label>
			<input type={input_type} name={name} id={id} node_ref={node_ref} autocomplete prop:required={required} class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500" placeholder={placeholder} />
    	</div>
	}
}