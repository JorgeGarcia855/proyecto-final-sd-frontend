use leptos::*;
use leptos_router::Form;

use crate::components::atoms::{custom_input::CustomInput, custom_button::CustomButton};

#[component]
pub fn SalesForm() -> impl IntoView {
    view! {
      <section class="bg-white dark:bg-gray-900 grow">
        <div class="py-8 px-4 mx-auto lg:py-8">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Hacer Venta"}</h2>
		<CheckClient/>
		<CheckProduct/>
		<CheckProduct/>
		<CheckProduct/>
        <Form action="">
            <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
                <CustomInput name="Total Venta" input_type="number" id="tven"/>
                <CustomInput name="Total IVA" input_type="number" id="tiva"/>
                <CustomInput name="Total con IVA" input_type="number" id="tviva"/>
				<CustomButton on_click=move |_| {}  label="Confirmar"/> 
            </div>
        </Form>
        </div>
      </section>
    }
}

#[component]
fn CheckClient() -> impl IntoView  {

	view! {
        <Form action="">
            <div class="grid gap-4 mx-auto  sm:grid-cols-8  sm:grid-rows-1 sm:gap-6">
                <CustomInput name="Cedula" input_type="number" id="ced"/>
                <CustomButton on_click=move |_| {}  label="Consultar"/> 
                <CustomInput name="Cliente" input_type="cliente" id="email"/>
                <CustomInput name="Consecutivo" input_type="num" id="cons"/> 
            </div>
        </Form>
	}
}

#[component]
fn CheckProduct() -> impl IntoView  {
	view! {
        <Form action="">
            <div class="grid gap-4 gap-4 sm:grid-cols-10 sm:grid-rows-1 sm:gap-6">
                <CustomInput name="Codigo Prod." input_type="number" id="ced"/>
                <CustomButton on_click=move |_| {}  label="Consultar"/> 	
                <CustomInput name="Nombre Prod." input_type="text" id="nom"/> 
				<CustomInput name="Cant." input_type="num" id="cant"/>  
				<CustomInput name="Valor Total" input_type="num" id="val"/> 
            </div>
        </Form>
	}
}