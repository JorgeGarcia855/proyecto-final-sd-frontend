use leptos::{*, html::Input};
use leptos_router::*;

use crate::components::{atoms::{custom_input::CustomInput, custom_button::CustomButton}, api::user::{ fetch_user, Usuarios, post_user, patch_user, delete_user}};

#[component]
pub fn UserForm() -> impl IntoView {
    let id_input: NodeRef<Input> = create_node_ref();
    let email_input: NodeRef<Input> = create_node_ref();
    let name_input: NodeRef<Input> = create_node_ref();
    let pass_input: NodeRef<Input> = create_node_ref();
    let user_input: NodeRef<Input> = create_node_ref();

	let on_consultar_click = move |_| {
        let current_id = id_input.get().expect("existing element").value().parse::<i64>();  //done
        match current_id {
            Ok(id) => {
                let user_id = create_rw_signal::<i64>(id);
                let user = create_resource(user_id, |id| async move { fetch_user(id).await });
                create_effect(move |_| {
                    match user() {
                        Some(usuario) => {
                            name_input.get().expect("existing element").set_value(&usuario.nombre);
                            email_input.get().expect("existing element").set_value(&usuario.email);
                            pass_input.get().expect("existing element").set_value(&usuario.password);
                            user_input.get().expect("existing element").set_value(&usuario.usuario);
                        },
                        None => {}
                    };
                });
            },
            Err(_) => gloo_dialogs::alert("there must be some id")
        }
	};

	let on_crear_click = move |_| {
        let cedula = id_input.get().expect("existing element").value();
        let nombre = name_input.get().expect("existing element").value();
        let email = email_input.get().expect("existing element").value();
        let password = pass_input.get().expect("existing element").value();
        let usuario = user_input.get().expect("existing element").value();

        if !cedula.is_empty() && !nombre.is_empty() && !email.is_empty() && !password.is_empty() && !usuario.is_empty() {
            let new_user = Usuarios { cedula: Some(cedula.parse().unwrap()), nombre, email, password, usuario};
            wasm_bindgen_futures::spawn_local(async move { post_user(new_user).await });
            gloo_dialogs::alert("Usuario creado");
        } else {
            gloo_dialogs::alert("you must fill the form");
        }
	};

	let on_actualizar_click = move |_| {
        let cedula = id_input.get().expect("existing element").value();
        let nombre = name_input.get().expect("existing element").value();
        let email = email_input.get().expect("existing element").value();
        let password = pass_input.get().expect("existing element").value();
        let usuario = user_input.get().expect("existing element").value();
        
        if !cedula.is_empty() && !nombre.is_empty() && !email.is_empty() && !password.is_empty() && !usuario.is_empty() {
            let existing_user = Usuarios { cedula: Some(cedula.parse().unwrap()), nombre, email, password, usuario};
            wasm_bindgen_futures::spawn_local(async move { patch_user(cedula.parse().unwrap(), existing_user).await });
            gloo_dialogs::alert("Usuario actualizado");
        } else {
            gloo_dialogs::alert("you must fill the form");
        }
	};

	let on_borrar_click = move |_| {
        let current_id = id_input.get().expect("existing element").value().parse::<i64>();  //done
        match current_id {
            Ok(id) => {
                wasm_bindgen_futures::spawn_local(async move { delete_user(id).await });
                gloo_dialogs::alert("Usuario borrado");
            },
            Err(_) => gloo_dialogs::alert("there must be some id")
        };
	};

	view! {
        <section class="bg-white dark:bg-gray-900 grow">
            <div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
                <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">"Crear Nuevo Usuario"</h2>
                <Form action="">
                    <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
                        <CustomInput name="Cedula" input_type="number" id="ced" placeholder="Ej: 123456" node_ref=id_input/>
                        <CustomInput name="Email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com" node_ref=email_input/>
                        <CustomInput name="Nombre" input_type="text" id="nom" placeholder="Ej: John Doe" node_ref=name_input/>
                        <CustomInput name="Contraseña" input_type="password" id="password" node_ref=pass_input/>
                        <CustomInput name="Nombre de Usuario" input_type="text" id="username" placeholder="Ej: john_doe123" node_ref=user_input/>
                    </div>
                    <div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
                        <CustomButton on_click=on_consultar_click id="consultar" label="Consultar"/>
                        <CustomButton on_click=on_crear_click id="crear" label="Crear"/>
                        <CustomButton on_click=on_actualizar_click id="actualizar" label="Actualizar"/>
                        <CustomButton on_click=on_borrar_click id="borrar" label="Borrar"/>
                    </div>
                </Form>
            </div>
        </section>
    }
}