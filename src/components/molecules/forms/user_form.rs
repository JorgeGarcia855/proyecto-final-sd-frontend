use gloo::console::log;
use leptos::*;


use crate::components::{atoms::{custom_input::CustomInput, custom_button::CustomButton}, api::user::{Usuarios, fetch_user}};

#[component]
pub fn UserForm() -> impl IntoView {
	let (user_id, set_user_id) = create_signal::<Option<i64>>(None);
	let user = create_resource(user_id, |id| fetch_user(id));
	


	view! {

		
		<section class="bg-white dark:bg-gray-900 grow">
		<div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
		<h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Usuario"}</h2>
		<form action="#">
			<div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
				
				<div class="sm:col-span-2">
					<label for="ced" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">"Cedula"</label>
					<input type="number" prop:value=move || {user.get().map(|us| match us {
						Some(us) => us.cedula.to_string(),
						None => "".to_string()
					})} name="Cedula" id="ced" autocomplete class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500" placeholder="Ej: 124545" required />
    			</div>




				<CustomInput name="Email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com" />
                <CustomInput name="Nombre" input_type="text" id="nom" placeholder="Ej: John Doe"/> 
                <CustomInput name="Contraseña" input_type="password" id="password"/>
                <CustomInput name="Nombre de Usuario" input_type="text" id="username" placeholder="Ej: john_doe123" />
			</div>
			<div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
				<CustomButton on_click=move |_| {set_user_id(Some(98760554))}  label="Consultar"/> 
				<CustomButton on_click=move |_| {}  label="Crear"/>
				<CustomButton on_click=move |_| {}  label="Actualizar"/>
				<CustomButton on_click=move |_| {}  label="Borrar"/>
				 
			</div>
			
		</form>
		</div>
		</section>
    }
}


