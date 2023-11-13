use leptos::*;

use crate::components::atoms::{custom_input::CustomInput, custom_button::CustomButton};

#[component]
pub fn ProviderForm() -> impl IntoView {
	view! {
		<section class="bg-white dark:bg-gray-900 grow">
		<div class="py-8 px-4 mx-auto max-w-2xl lg:py-16">
		<h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Crear Nuevo Cliente"}</h2>
		<form action="#">
			<div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
				<CustomInput name="NIT" input_type="number" id="nit" placeholder="Ej: 1234567"/>
                <CustomInput name="Ciudad" input_type="text" id="ciudad" placeholder="Ej: Bogota D.C."/>
				<CustomInput name="Direccion" input_type="text" id="dir" placeholder="Ej: Carrera 9b #132-23"/>
                <CustomInput name="Nombre" input_type="text" id="nom" placeholder="Ej: Empresa SAS"/> 
                <CustomInput name="Telefono" input_type="text" id="tel" placeholder="Ej: 302 578 9302"/>
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