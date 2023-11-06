use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    //pub onclick: Callback<MouseEvent>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cliente {
    pub cedula: i64,
    pub direccion: String,
    pub email: String,
    pub nombre: String,
    pub telefono: String,
}

#[function_component(CustomButton)]
pub fn custom_button(props: &Props) -> Html {
   
    html! {
          <button type="submit"  class="inline-flex items-center px-5 py-2.5 mt-4 sm:mt-6 text-sm font-medium text-center text-white bg-blue-500 rounded-lg focus:ring-4 focus:ring-primary-200 dark:focus:ring-primary-900 hover:bg-primary-800">{props.label.clone()}</button>
    }
}
