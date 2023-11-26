use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DetalleVentas {
    pub codigo: Option<i64>,
    pub codigo_producto: Option<i64>,
    pub codigo_venta: Option<i64>,
    pub cantidad_producto: i32,
    pub valor_total: f64,
    pub valor_venta: f64,
    pub valor_iva: f64,
}

const API: &str = "http://localhost:8080/api/detalle_ventas/";

// pub async fn fetch_sale_det(id: i64) -> DetalleVentas {	
// 	let mut det_venta = DetalleVentas::default();
// 	let fetched_result = get(format!("{API}{id}").as_str()).await;
// 	if let Ok(res) = fetched_result {
// 		let json = res.json().await;
// 		match json {
// 			Ok(val) => { det_venta = val },
// 			Err(_) => { gloo_dialogs::alert(format!("no sale detail with this id").as_str()) }
// 		}
// 	}
// 	det_venta
// }

// pub async fn fetch_sales_det() -> Option<Vec<DetalleVentas>> {
// 	let fetch: Vec<DetalleVentas> = reqwest::get(API)
//             .await
//             .unwrap()
//             .json()
//             .await
//             .unwrap();
// 	Some(fetch)
// }

pub async fn post_sale_det(sale_det: DetalleVentas)  {
	let client = Client::new();
	let _ = client.post(API)
		.json(&sale_det)
		.send()
		.await;
	
}

// pub async fn patch_sale_det(id: i64, sale_det: DetalleVentas) {
// 	let client = Client::new();
// 	let _ = client.patch(format!("{API}{id}").as_str())
// 		.json(&sale_det)
// 		.send()
// 		.await;
// }

// pub async fn delete_sale_det(id: i64) {
// 	let client = Client::new();	
// 	let _ = client.delete(format!("{API}{id}").as_str())
// 		.send()
// 		.await;
// }