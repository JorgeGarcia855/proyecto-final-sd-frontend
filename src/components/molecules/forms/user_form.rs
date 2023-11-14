use leptos::*;


use crate::components::{atoms::{custom_input::CustomInput, custom_button::CustomButton}, api::user:: fetch_user};

#[component]
pub fn UserForm() -> impl IntoView {
	let (user_id, set_user_id) = create_signal::<Option<i64>>(None);
	let user = create_resource(user_id, |id| fetch_user(id));
	
	let val = Box::new(move || {
					user.get().map(|us| match us {
						Some(us) => us.cedula.to_string(),
						None => "".to_string()
					})
				});	

	view! {
		<section class="bg-white dark:bg-gray-900 grow">
		<div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
		<h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Usuario"}</h2>
		<form action="#">
			<div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
				<CustomInput name="Cedula" input_type="number" id="ced" placeholder="Ej: 123456" value=val/>	
				<CustomInput name="Email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com"/>
                <CustomInput name="Nombre" input_type="text" id="nom" placeholder="Ej: John Doe"/> 
                <CustomInput name="Contraseña" input_type="password" id="password"/>
                <CustomInput name="Nombre de Usuario" input_type="text" id="username" placeholder="Ej: john_doe123" />
			</div>
			<div class="grid gap-4 sm:grid-cols-4 sm:grid-rows-1 sm:gap-6">
				<CustomButton on_click=move |_| {}  label="Consultar"/> 
				<CustomButton on_click=move |_| {}  label="Crear"/>
				<CustomButton on_click=move |_| {}  label="Actualizar"/>
				<CustomButton on_click=move |_| {}  label="Borrar"/>
			</div>
		</form>
		</div>
		</section>
    }
}


