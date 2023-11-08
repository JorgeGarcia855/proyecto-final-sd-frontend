use gloo::console::log;
use reqwasm::http::Request;
use wasm_bindgen_futures::js_sys::JsString;
use yew::prelude::*;

use crate::components::atoms::{
    custom_button::CustomButton, custom_input::CustomInput, entities::Clientes,
};

#[function_component(ClientForm)]
pub fn client_form() -> Html {
    let cliente = use_state(|| None);
    let cliente_clone = cliente.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let fetch: Clientes = Request::get("http://localhost:8080/api/clientes/98765")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        // log!(&JsString::from(serde_json::to_string(&fetch).unwrap()));
        cliente.set(Some(fetch));
    });

    match cliente_clone.as_ref() {
        Some(cliente) => html! {
          <section class="bg-white dark:bg-gray-900 grow">
            <div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
            <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Cliente"}</h2>
            <form action="#">
                <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
                    <CustomInput name="Cedula"  input_name="cedula" input_type="number" id="ced" placeholder="Ej: 45367284"/>
                    <CustomInput name="Direccion" value={cliente.direccion.clone()} input_name="direccion" input_type="text" id="dir" placeholder="Ej: Carrera 9b #132-23"/>
                    <CustomInput name="Email" input_name="email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com"/>
                    <CustomInput name="Nombre" input_name="nombre" input_type="text" id="nom" placeholder="Ej: John Doe"/>
                    <CustomInput name="Telefono" input_name="telefono" input_type="text" id="tel" placeholder="Ej: 302 578 9302"/>
                </div>
                <div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
                    <CustomButton label="Consultar"/>
                    <CustomButton label="Crear"/>
                    <CustomButton label="Actualizar"/>
                    <CustomButton label="Borrar"/>
                </div>
            </form>
            </div>
          </section>
        },
        None => html! {
          <section class="bg-white dark:bg-gray-900 grow">
            <div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
            <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Cliente"}</h2>
            <form action="#">
                <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
                    <CustomInput name="Cedula" input_name="cedula" input_type="number" id="ced" placeholder="Ej: 45367284"/>
                    <CustomInput name="Direccion" input_name="direccion" input_type="text" id="dir" placeholder="Ej: Carrera 9b #132-23"/>
                    <CustomInput name="Email" input_name="email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com"/>
                    <CustomInput name="Nombre" input_name="nombre" input_type="text" id="nom" placeholder="Ej: John Doe"/>
                    <CustomInput name="Telefono" input_name="telefono" input_type="text" id="tel" placeholder="Ej: 302 578 9302"/>
                </div>
                <div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
                    <CustomButton label="Consultar"/>
                    <CustomButton label="Crear"/>
                    <CustomButton label="Actualizar"/>
                    <CustomButton label="Borrar"/>
                </div>
            </form>
            </div>
          </section>
        },
    }

    // html! {
    //   <section class="bg-white dark:bg-gray-900 grow">
    //     <div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
    //     <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Cliente"}</h2>
    //     <form action="#">
    //         <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
    //             <CustomInput name="Cedula"  input_name="cedula" input_type="number" id="ced" placeholder="Ej: 45367284"/>
    //             <CustomInput name="Direccion" input_name="direccion" input_type="text" id="dir" placeholder="Ej: Carrera 9b #132-23"/>
    //             <CustomInput name="Email" input_name="email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com"/>
    //             <CustomInput name="Nombre" input_name="nombre" input_type="text" id="nom" placeholder="Ej: John Doe"/>
    //             <CustomInput name="Telefono" input_name="telefono" input_type="text" id="tel" placeholder="Ej: 302 578 9302"/>
    //         </div>
    //         <div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
    // 			<CustomButton label="Consultar"/>
    // 			<CustomButton label="Crear"/>
    // 			<CustomButton label="Actualizar"/>
    // 			<CustomButton label="Borrar"/>
    //         </div>
    //     </form>
    //     </div>
    //   </section>
    // }
}
