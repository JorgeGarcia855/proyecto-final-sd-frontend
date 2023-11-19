use leptos::{*, html::Input};
use leptos_router::Form;

use crate::components::{atoms::{custom_input::CustomInput, custom_button::CustomButton}, api::client::{fetch_client, Clientes, post_client, patch_client, delete_client}};

#[component]
pub fn ClientForm() -> impl IntoView {
	let id_input: NodeRef<Input> = create_node_ref();
    let dir_input: NodeRef<Input> = create_node_ref();
	let email_input: NodeRef<Input> = create_node_ref();
    let name_input: NodeRef<Input> = create_node_ref();
    let tel_input: NodeRef<Input> = create_node_ref();
	
	let consultar = move |_| {
		let current_id = id_input.get().expect("existing element").value().parse::<i64>();  //done
        if let Ok(id) = current_id {
            let client = create_resource(create_rw_signal::<i64>(id), |id| async move { fetch_client(id).await });
            create_effect(move |_| {
                if let Some(usuario) = client.get() {
                    dir_input.get().expect("existing element").set_value(&usuario.direccion);
                    email_input.get().expect("existing element").set_value(&usuario.email);
                    name_input.get().expect("existing element").set_value(&usuario.nombre);
                    tel_input.get().expect("existing element").set_value(&usuario.telefono);
                }
            });
        } else { gloo_dialogs::alert("there must be some id") }
	};

	let crear = move |_| {
		let cedula = id_input.get().expect("existing element").value();
        let direccion = dir_input.get().expect("existing element").value();
        let email = email_input.get().expect("existing element").value();
        let nombre = name_input.get().expect("existing element").value();
        let telefono = tel_input.get().expect("existing element").value();

        if !cedula.is_empty() && !direccion.is_empty() && !email.is_empty() && !nombre.is_empty() && !telefono.is_empty() {
            let new_client = Clientes { cedula: Some(cedula.parse().unwrap()), direccion, email, nombre, telefono };
            wasm_bindgen_futures::spawn_local(async move { post_client(new_client).await });
            gloo_dialogs::alert("Cliente creado");
        } else {
            gloo_dialogs::alert("you must fill the form");
        }
	};

	let actualizar = move |_| {
		let cedula = id_input.get().expect("existing element").value();
        let direccion = dir_input.get().expect("existing element").value();
        let email = email_input.get().expect("existing element").value();
        let nombre = name_input.get().expect("existing element").value();
        let telefono = tel_input.get().expect("existing element").value();

        if !cedula.is_empty() && !direccion.is_empty() && !email.is_empty() && !nombre.is_empty() && !telefono.is_empty() {
            let existing_client = Clientes { cedula: Some(cedula.parse().unwrap()), direccion, email, nombre, telefono };
            wasm_bindgen_futures::spawn_local(async move { patch_client(cedula.parse().unwrap(), existing_client).await });
            gloo_dialogs::alert("Cliente actualizado");
        } else {
            gloo_dialogs::alert("you must fill the form");
        }
	};

	let borrar = move |_| {
		let current_id = id_input.get().expect("existing element").value().parse::<i64>();
        match current_id {
            Ok(id) => {
                wasm_bindgen_futures::spawn_local(async move { delete_client(id).await });
                gloo_dialogs::alert("Cliente borrado");
            },
            Err(_) => gloo_dialogs::alert("there must be some id")
        };
	};

	view! {
		<section class="bg-white dark:bg-gray-900 grow">
		<div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
		<h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Cliente"}</h2>
		<Form action="">
			<div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
				<CustomInput name="Cedula" input_type="number" id="ced" placeholder="Ej: 45367284" node_ref=id_input/>
				<CustomInput name="Direccion" input_type="text" id="dir" placeholder="Ej: Carrera 9b #132-23" node_ref=dir_input/>
				<CustomInput name="Email"  input_type="email" id="email" placeholder="Ej: nombre.123@correo.com" node_ref=email_input/>
				<CustomInput name="Nombre"  input_type="text" id="nom" placeholder="Ej: John Doe" node_ref=name_input/>
				<CustomInput name="Telefono" input_type="text" id="tel" placeholder="Ej: 302 578 9302" node_ref=tel_input/>
			</div>
			<div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
				<CustomButton on_click=consultar id="consultar" label="Consultar"/>
				<CustomButton on_click=crear id="crear" label="Crear"/>
				<CustomButton on_click=actualizar id="actualizar" label="Actualizar"/>
				<CustomButton on_click=borrar id="borrar" label="Borrar"/>
			</div>
		</Form>
		</div>
		</section>
    }
}