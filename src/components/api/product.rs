use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Productos {
    pub codigo: Option<i64>,
    pub nit_proveedor: Option<i64>,
    pub iva_compra: f64,
    pub nombre_producto: String,
    pub precio_compra: f64,
    pub precio_venta: f64,
}
const API: &str = "http://localhost:8080/api/productos/";

pub async fn fetch_prod(id: i64) -> Productos {	
	let mut producto = Productos::default();
	let fetched_result = get(format!("{API}{id}").as_str()).await;
	if let Ok(res) = fetched_result {
		let json = res.json().await;
		match json {
			Ok(val) => { producto = val },
			Err(_) => { gloo_dialogs::alert(format!("no product with this id").as_str()) }
		}
	}
	producto
}

pub async fn fetch_prods() -> Option<Vec<Productos>> {
	let fetch: Vec<Productos> = reqwest::get(API)
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
	Some(fetch)
}

pub async fn post_prod(prod: Productos)  {
	let client = Client::new();
	let _ = client.post(API)
		.json(&prod)
		.send()
		.await;
	
}

pub async fn patch_prod(id: i64, prod: Productos) {
	let client = Client::new();
	let _ = client.patch(format!("{API}{id}").as_str())
		.json(&prod)
		.send()
		.await;
}

pub async fn delete_prod(id: i64) {
	let client = Client::new();	
	let _ = client.delete(format!("{API}{id}").as_str())
		.send()
		.await;
}