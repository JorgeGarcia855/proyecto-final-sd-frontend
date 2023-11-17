use leptos::*;
use reqwest::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usuarios {
    pub cedula: i64,
    pub email: String,
    pub nombre: String,
    pub password: String,
    pub usuario: String,
}

// impl Default for Usuarios {
//     fn default() -> Self {
//         Self { cedula: Default::default(), email: Default::default(), nombre: Default::default(), password: Default::default(), usuario: Default::default() }
//     }
// }

pub async fn fetch_user(id: Option<i64>) -> Option<Usuarios> {
	match id {
		Some(id) => {
			let user = reqwest::get(format!("http://localhost:8080/api/usuarios/{}", id).as_str())
				.await
				.unwrap()
				.json::<Usuarios>()
				.await
				.unwrap();
			Some(user)
		},
		None => None
	}
}

pub async fn fetch_users() -> Option<Vec<Usuarios>> {
	let fetch = reqwest::get("http://localhost:8080/api/usuarios/")
		.await
		.unwrap()
		.json::<Vec<Usuarios>>()
		.await
		.unwrap();
	Some(fetch)
}

pub async fn post_user(user: Usuarios)  {
	let client = Client::new();
	let _ = client.post("http://localhost:8080/api/usuarios/")
		.json(&user)
		.send()
		.await;
	
}

