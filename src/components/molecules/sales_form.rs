use yew::prelude::*;

use crate::components::atoms::{custom_input::CustomInput, custom_button::CustomButton};

#[function_component(SalesForm)]
pub fn sales_form() -> Html {
    html! {
      <section class="bg-white dark:bg-gray-900 grow">
        <div class="py-8 px-4 mx-auto lg:py-8">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">{"Hacer Venta"}</h2>
		<CheckClient/>
		<CheckProduct/>
		<CheckProduct/>
		<CheckProduct/>
        <form action="#">
            <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
                <CustomInput name="Total Venta" input_name="tven" input_type="number" id="tven"/>
                <CustomInput name="Total IVA" input_name="tiva" input_type="number" id="tiva"/>
                <CustomInput name="Total con IVA" input_name="tviva" input_type="number" id="tviva"/>
				<CustomButton label="Confirmar"/> 
            </div>
        </form>
        </div>
      </section>
    }
}

#[function_component(CheckClient)]
fn check_client() -> Html {

	html! {
        <form action="#">
            <div class="grid gap-4 mx-auto  sm:grid-cols-8  sm:grid-rows-1 sm:gap-6">
                <CustomInput name="Cedula" input_name="cedula" input_type="number" id="ced"/>
                <CustomButton label="Consultar"/> 
                <CustomInput name="Cliente" input_name="cliente" input_type="cliente" id="email"/>
                <CustomInput name="Consecutivo" input_name="cons" input_type="num" id="cons"/> 
            </div>
        </form>
	}
}

#[function_component(CheckProduct)]
fn check_product() -> Html {
	html! {
        <form action="#">
            <div class="grid gap-4 gap-4 sm:grid-cols-10 sm:grid-rows-1 sm:gap-6">
                <CustomInput name="Codigo Prod." input_name="cedula" input_type="number" id="ced"/>
                <CustomButton label="Consultar"/> 	
                <CustomInput name="Nombre Prod." input_name="nombre" input_type="text" id="nom"/> 
				<CustomInput name="Cant." input_name="cant" input_type="num" id="cant"/>  
				<CustomInput name="Valor Total" input_name="val" input_type="num" id="val"/> 
            </div>
        </form>
	}
}