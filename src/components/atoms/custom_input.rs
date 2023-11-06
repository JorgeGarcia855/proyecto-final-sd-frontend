use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub input_type: String,
    pub input_name: String,
    pub id: String,
    pub placeholder: String, 
}

#[function_component(CustomInput)] 
pub fn custom_input(props: &Props) -> Html {

  html! {
    <div class="sm:col-span-2">
      <label for={props.id.clone()} class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{props.name.clone()}</label>
      <input type={props.input_type.clone()} name={props.name.clone()} id={props.id.clone()} class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500" placeholder={props.placeholder.clone()} required=true />
    </div>
  }
}