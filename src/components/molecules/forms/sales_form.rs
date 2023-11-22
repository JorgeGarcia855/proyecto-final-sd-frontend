use leptos::{*, html::Input};
use leptos_router::Form;

use crate::components::{atoms::{custom_input::CustomInput, custom_button::CustomButton}, api::{client::fetch_client, product::{fetch_prod, Productos}, sales::{Ventas, post_sale}, det_sales::{DetalleVentas, post_sale_det}}};

#[component]
pub fn SalesForm() -> impl IntoView {
    let client_id_input: NodeRef<Input> = create_node_ref();
    let client_name_input: NodeRef<Input> = create_node_ref();
    let cons_input: NodeRef<Input> = create_node_ref();

    let prod_id_input: [NodeRef<Input>; 3] = [create_node_ref(),create_node_ref(),create_node_ref()];
    let prod_name_input: [NodeRef<Input>; 3] = [create_node_ref(),create_node_ref(),create_node_ref()];
    let cant_input: [NodeRef<Input>; 3] = [create_node_ref(),create_node_ref(),create_node_ref()];
    let prod_val_input: [NodeRef<Input>; 3] = [create_node_ref(),create_node_ref(),create_node_ref()];

    let prod_venta_input: [NodeRef<Input>; 3] = [create_node_ref(),create_node_ref(),create_node_ref()];
    let prod_iva_input: [NodeRef<Input>; 3] = [create_node_ref(),create_node_ref(),create_node_ref()];

    let total_venta_input: NodeRef<Input> = create_node_ref();
    let total_iva_input: NodeRef<Input> = create_node_ref();
    let total_venta_iva_input: NodeRef<Input> = create_node_ref();

    
    let consecutivo = create_rw_signal::<i64>(0);

    let confirmar = move |_| {
        let client_name = client_name_input.get().unwrap().value();
        let prod_name_1 = prod_name_input[0].get().unwrap().value();
        let prod_name_2 = prod_name_input[1].get().unwrap().value();
        let prod_name_3 = prod_name_input[2].get().unwrap().value();
        let prod_val_1 = prod_val_input[0].get().unwrap().value();
        let prod_val_2 = prod_val_input[1].get().unwrap().value();
        let prod_val_3 = prod_val_input[2].get().unwrap().value();

        if !client_name.is_empty() && !prod_name_1.is_empty() && !prod_name_2.is_empty() && !prod_name_3.is_empty() && !prod_val_1.is_empty() && !prod_val_2.is_empty() && !prod_val_3.is_empty() {
            let client_id = client_id_input.get().unwrap().value().parse::<i64>().unwrap();
            let prod_id_1 = prod_id_input[0].get().unwrap().value().parse::<i64>().unwrap();
            let prod_id_2 = prod_id_input[1].get().unwrap().value().parse::<i64>().unwrap();
            let prod_id_3 = prod_id_input[2].get().unwrap().value().parse::<i64>().unwrap();

            let prod_val_1 = prod_val_input[0].get().unwrap().value().parse::<f64>();
            let prod_val_2 = prod_val_input[1].get().unwrap().value().parse::<f64>();
            let prod_val_3 = prod_val_input[2].get().unwrap().value().parse::<f64>();

            match (prod_val_1, prod_val_2, prod_val_3) {
                (Ok(val1), Ok(val2), Ok(val3)) => {
                    let prod_iva_1 = prod_iva_input[0].get().unwrap().value().parse::<f64>().unwrap();
                    let prod_iva_2 = prod_iva_input[1].get().unwrap().value().parse::<f64>().unwrap();
                    let prod_iva_3 = prod_iva_input[2].get().unwrap().value().parse::<f64>().unwrap();

                    let cant1 = cant_input[0].get().unwrap().value().parse::<i32>().unwrap();
                    let cant2 = cant_input[1].get().unwrap().value().parse::<i32>().unwrap();
                    let cant3 = cant_input[2].get().unwrap().value().parse::<i32>().unwrap();

                    let valor_venta = val1 + val2 + val3;
                    let iva_venta = (prod_iva_1 * cant1 as f64) + (prod_iva_2 * cant2 as f64) + (prod_iva_3 * cant3 as f64);
                    let total_venta = valor_venta + iva_venta;

                    total_venta_input.get().unwrap().set_value(format!("{}", valor_venta).as_str());
                    total_iva_input.get().unwrap().set_value(format!("{}", iva_venta).as_str());
                    total_venta_iva_input.get().unwrap().set_value(format!("{}", total_venta).as_str());

                    consecutivo.update(|n| *n += 1);
                    let cons = consecutivo.get();
                    cons_input.get().unwrap().set_value(cons.to_string().as_str());
                    let new_sale = Ventas { 
                        codigo:Some(cons), 
                        cedula_cliente: Some(client_id), 
                        cedula_usuario: Some(client_id), 
                        iva_venta, 
                        total_venta, 
                        valor_venta 
                    };
                    
                    let new_det_sale1 = DetalleVentas { 
                        codigo:Some(cons+10), 
                        codigo_producto: Some(prod_id_1),
                        codigo_venta: Some(cons), 
                        cantidad_producto: cant1, 
                        valor_venta: val1, 
                        valor_total: val1 + (prod_iva_1 * cant1 as f64), 
                        valor_iva: prod_iva_1 * cant1 as f64 
                    }; 

                    let new_det_sale2 = DetalleVentas { 
                        codigo:Some(cons+20), 
                        codigo_producto: Some(prod_id_2),
                        codigo_venta: Some(cons), 
                        cantidad_producto: cant2, 
                        valor_venta: val2, 
                        valor_total: val2 + (prod_iva_2 * cant2 as f64), 
                        valor_iva: prod_iva_2 * cant2 as f64 
                    }; 

                    let new_det_sale3 = DetalleVentas { 
                        codigo:Some(cons+30), 
                        codigo_producto: Some(prod_id_3),
                        codigo_venta: Some(cons), 
                        cantidad_producto: cant3, 
                        valor_venta: val3, 
                        valor_total: val3 + (prod_iva_3 * cant3 as f64), 
                        valor_iva: prod_iva_3 * cant3 as f64 
                    }; 

                    wasm_bindgen_futures::spawn_local(async move {
                        post_sale(new_sale).await;
                        post_sale_det(new_det_sale1).await;
                        post_sale_det(new_det_sale2).await;
                        post_sale_det(new_det_sale3).await;
                    });
                    gloo_dialogs::alert("venta realizada");
                },
                _ => {
                    gloo_dialogs::alert("invalid values")
                }
            }    
        } else {
            // consecutivo.update(|n| *n += 1);
            // gloo_dialogs::alert(format!("{}", consecutivo.get()).as_str());
            gloo_dialogs::alert("you must fill all the forms")
        }

    };

    view! {
        <section class="bg-white dark:bg-gray-900 grow">
        <div class="py-8 px-4 mx-auto lg:py-8">
        <h2 class="mb-4 text-xl font-bold text-gray-900 dark:text-white">"Hacer Venta"</h2>
        <CheckClient id=client_id_input name=client_name_input cons=cons_input/>
        <CheckProduct id=prod_id_input[0] name=prod_name_input[0] cant=cant_input[0] valor=prod_val_input[0] venta=prod_venta_input[0] iva=prod_iva_input[0]/>
        <CheckProduct id=prod_id_input[1] name=prod_name_input[1] cant=cant_input[1] valor=prod_val_input[1] venta=prod_venta_input[1] iva=prod_iva_input[1]/>
        <CheckProduct id=prod_id_input[2] name=prod_name_input[2] cant=cant_input[2] valor=prod_val_input[2] venta=prod_venta_input[2] iva=prod_iva_input[2]/>
            <Form action="">
            <div class="grid gap-4 sm:grid-cols-2 sm:gap-6">
                <CustomInput name="Total Venta" input_type="number" id="tven" disabled=true node_ref=total_venta_input/>
                <CustomInput name="Total IVA" input_type="number" id="tiva" disabled=true node_ref=total_iva_input/>
                <CustomInput name="Total con IVA" input_type="number" id="tviva" disabled=true node_ref=total_venta_iva_input/>
                <CustomButton on_click=confirmar label="Confirmar"/> 
            </div>
            </Form>
        </div>
        </section>
    }
}

#[component]
fn CheckClient(
    id: NodeRef<Input>,
    name: NodeRef<Input>,
    cons: NodeRef<Input>
) -> impl IntoView  {
    let consultar = move |_| {
        let current_id = id.get().expect("existing element").value().parse::<i64>();
        if let Ok(id) = current_id {
            let client = create_resource(create_rw_signal::<i64>(id), |id| async move { fetch_client(id).await });
            create_effect(move |_| {
                if let Some(cliente) = client.get() {
                    name.get().expect("existing element").set_value(&cliente.nombre);
                }
            });
        } else { gloo_dialogs::alert("there must be some id") }
    };

	view! {
        <Form action="">
            <div class="grid gap-4 mx-auto  sm:grid-cols-8  sm:grid-rows-1 sm:gap-6">
                <CustomInput name="Cedula" input_type="number" id="ced" node_ref=id/>
                <CustomButton on_click=consultar label="Consultar"/> 
                <CustomInput name="Cliente" input_type="cliente" id="email" disabled=true node_ref=name/>
                <CustomInput name="Consecutivo" input_type="num" id="cons" disabled=true node_ref=cons/> 
            </div>
        </Form>
	}
}

#[component]
fn CheckProduct(
    id: NodeRef<Input>,
    name: NodeRef<Input>,
    cant: NodeRef<Input>,
    valor: NodeRef<Input>,
    venta: NodeRef<Input>,
    iva: NodeRef<Input>,
) -> impl IntoView  {
    let consultar = move |_| {
        let current_id = id.get().expect("existing element").value().parse::<i64>();
        if let Ok(id) = current_id {
            let product = create_resource(create_rw_signal::<i64>(id), |id| async move { fetch_prod(id).await });
            create_effect(move |_| {
                if let Some(producto) = product.get() {
                    name.get().expect("existing element").set_value(&producto.nombre_producto);
                    venta.get().expect("existing").set_value(*&producto.precio_venta.to_string().as_str());
                    iva.get().expect("existing").set_value(*&producto.iva_compra.to_string().as_str());
                }
            });
        } else { gloo_dialogs::alert("there must be some id") }
    };

    let input = move |_| {
        let num = cant.get().expect("input field").value().parse::<i32>(); 
        let valor = valor.get().expect("input field");
        let name = name.get().expect("input field").value();

        if !name.is_empty() {
            if let Ok(n) = num {
                let precio = venta.get().expect("existing").value().parse::<f64>().unwrap();
                if n == 0 {
                    valor.set_value("field cannot be 0");
                } else {
                    valor.set_value(format!("{}", n as f64 * precio).as_str());
                }
            } else {
                valor.set_value("field cannot be empty");
            }
        } else {
            valor.set_value("you must make the query");
        }
    };

	view! {
        <Form action="">
            <div class="grid gap-4 gap-4 sm:grid-cols-10 sm:grid-rows-1 sm:gap-6">
                <CustomInput name="Codigo Prod." input_type="number" id="ced" node_ref=id/>
                <CustomButton on_click=consultar label="Consultar"/> 	
                <CustomInput name="Nombre Prod." input_type="text" id="nom" disabled=true node_ref=name/> 
				<CustomInput name="Cant." input_type="number" id="cant" on_input=Box::new(input) node_ref=cant/>  
				<CustomInput name="Valor Total" input_type="text" id="val" disabled=true node_ref=valor/>
                <CustomInput input_type="hidden" id="real_val" disabled=true node_ref=venta/> 
                <CustomInput input_type="hidden" id="real_iva" disabled=true node_ref=iva/>  
            </div>
        </Form>
	}
}