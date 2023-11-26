use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Ventas {
    pub codigo: Option<i64>,
    pub cedula_cliente: Option<i64>,
    pub cedula_usuario: Option<i64>,
    pub iva_venta: f64,
    pub total_venta: f64,
    pub valor_venta: f64,
}
const API: &str = "http://localhost:8080/api/ventas/";

// pub async fn fetch_sale(id: i64) -> Ventas {	
// 	let mut venta = Ventas::default();
// 	let fetched_result = get(format!("{API}{id}").as_str()).await;
// 	if let Ok(res) = fetched_result {
// 		let json = res.json().await;
// 		match json {
// 			Ok(val) => { venta = val },
// 			Err(_) => { gloo_dialogs::alert(format!("no sale with this id").as_str()) }
// 		}
// 	}
// 	venta
// }

pub async fn fetch_sales() -> Option<Vec<Ventas>> {
	let fetch: Vec<Ventas> = reqwest::get(API)
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
	Some(fetch)
}

pub async fn post_sale(sale: Ventas)  {
	let client = Client::new();
	let _ = client.post(API)
		.json(&sale)
		.send()
		.await;
	
}

// pub async fn patch_sale(id: i64, sale: Ventas) {
// 	let client = Client::new();
// 	let _ = client.patch(format!("{API}{id}").as_str())
// 		.json(&sale)
// 		.send()
// 		.await;
// }

// pub async fn delete_sale(id: i64) {
// 	let client = Client::new();	
// 	let _ = client.delete(format!("{API}{id}").as_str())
// 		.send()
// 		.await;
// }