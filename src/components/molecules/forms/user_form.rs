use leptos::*;

use crate::components::atoms::{custom_input::CustomInput, custom_button::CustomButton};

#[component]
pub fn UserForm() -> impl IntoView {
	view! {
		<section class="bg-white dark:bg-gray-900 grow">
		<div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
		<h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Cliente"}</h2>
		<form action="#">
			<div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
				<CustomInput name="Cedula" input_type="number" id="ced" placeholder="Ej: 45367284"/>
				<CustomInput name="Email" input_type="email" id="email" placeholder="Ej: nombre.123@correo.com"/>
                <CustomInput name="Nombre" input_type="text" id="nom" placeholder="Ej: John Doe"/> 
                <CustomInput name="Contraseña" input_type="password" id="password"/>
                <CustomInput name="Nombre de Usuario" input_type="text" id="username" placeholder="Ej: john_doe123"/>
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
    }
}