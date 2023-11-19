use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usuarios {
    pub cedula: Option<i64>,
    pub email: String,
    pub nombre: String,
    pub password: String,
    pub usuario: String,
}

const API: &str = "http://localhost:8080/api/usuarios/";

pub async fn fetch_user(id: i64) -> Usuarios {	
	let mut usuario = Usuarios::default();
	let fetched_result = get(format!("{API}{id}").as_str()).await;
	if let Ok(res) = fetched_result {
		let json = res.json().await;
		match json {
			Ok(val) => { usuario = val },
			Err(_) => { gloo_dialogs::alert(format!("no user with this id").as_str()) }
		}
	}
	usuario
}

pub async fn fetch_users() -> Option<Vec<Usuarios>> {
	let fetch = get(API)
		.await
		.unwrap()
		.json::<Vec<Usuarios>>()
		.await
		.unwrap();
	Some(fetch)
}

pub async fn post_user(user: Usuarios)  {
	let client = Client::new();
	let _ = client.post(API)
		.json(&user)
		.send()
		.await;
	
}

pub async fn patch_user(id: i64, user: Usuarios) {
	let client = Client::new();
	let _ = client.patch(format!("{API}{id}").as_str())
		.json(&user)
		.send()
		.await;
}

pub async fn delete_user(id: i64) {
	let client = Client::new();	
	let _ = client.delete(format!("{API}{id}").as_str())
		.send()
		.await;
}