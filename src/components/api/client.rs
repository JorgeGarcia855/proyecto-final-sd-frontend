use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Clientes {
    pub cedula: Option<i64>,
    pub direccion: String,
    pub email: String,
    pub nombre: String,
    pub telefono: String,
}

const API: &str = "http://localhost:8080/api/clientes/";

pub async fn fetch_client(id: i64) -> Clientes {	
	let mut cliente = Clientes::default();
	let fetched_result = get(format!("{API}{id}").as_str()).await;
	if let Ok(res) = fetched_result {
		let json = res.json().await;
		match json {
			Ok(val) => { cliente = val },
			Err(_) => { gloo_dialogs::alert(format!("no client with this id").as_str()) }
		}
	}
	cliente
}

pub async fn fetch_clients() -> Option<Vec<Clientes>> {
	let fetch: Vec<Clientes> = reqwest::get(API)
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
	Some(fetch)
}

pub async fn post_client(cli: Clientes)  {
	let client = Client::new();
	let _ = client.post(API)
		.json(&cli)
		.send()
		.await;
	
}

pub async fn patch_client(id: i64, cli: Clientes) {
	let client = Client::new();
	let _ = client.patch(format!("{API}{id}").as_str())
		.json(&cli)
		.send()
		.await;
}

pub async fn delete_client(id: i64) {
	let client = Client::new();	
	let _ = client.delete(format!("{API}{id}").as_str())
		.send()
		.await;
}