use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Proveedores {
    pub nit: Option<i64>,
    pub ciudad: String,
    pub direccion: String,
    pub nombre: String,
    pub telefono: String,
}

const API: &str = "http://localhost:8080/api/proveedores/";

pub async fn fetch_prov(id: i64) -> Proveedores {	
	let mut proveedor = Proveedores::default();
	let fetched_result = get(format!("{API}{id}").as_str()).await;
	if let Ok(res) = fetched_result {
		let json = res.json().await;
		match json {
			Ok(val) => { proveedor = val },
			Err(_) => { gloo_dialogs::alert(format!("no provider with this id").as_str()) }
		}
	}
	proveedor
}

// pub async fn fetch_provs() -> Option<Vec<Proveedores>> {
// 	let fetch: Vec<Proveedores> = reqwest::get(API)
//             .await
//             .unwrap()
//             .json()
//             .await
//             .unwrap();
// 	Some(fetch)
// }

pub async fn post_prov(prov: Proveedores)  {
	let client = Client::new();
	let _ = client.post(API)
		.json(&prov)
		.send()
		.await;
	
}

pub async fn patch_prov(id: i64, prov: Proveedores) {
	let client = Client::new();
	let _ = client.patch(format!("{API}{id}").as_str())
		.json(&prov)
		.send()
		.await;
}

pub async fn delete_prov(id: i64) {
	let client = Client::new();	
	let _ = client.delete(format!("{API}{id}").as_str())
		.send()
		.await;
}