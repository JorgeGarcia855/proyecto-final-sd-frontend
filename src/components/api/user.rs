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

pub async fn fetch_user(id: i64) -> Usuarios {	
	let mut usuario = Usuarios::default();
	let fetched_user = get(format!("http://localhost:8080/api/usuarios/{id}").as_str()).await;
	match fetched_user {
		Ok(res) => {
			let json = res.json::<Usuarios>().await;
			match json {
				Ok(user) => { usuario = user },
				Err(_) => { gloo_dialogs::alert("no user with this id") }
			}
		},
		Err(_) => { gloo_dialogs::alert("no user with this id") }
	};
	usuario
}

pub async fn fetch_users() -> Option<Vec<Usuarios>> {
	let fetch = get("http://localhost:8080/api/usuarios/")
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

pub async fn patch_user(id: i64, user: Usuarios) {
	let client = Client::new();
	let _ = client.patch(format!("http://localhost:8080/api/usuarios/{id}").as_str())
		.json(&user)
		.send()
		.await;
}

pub async fn delete_user(id: i64) {
	let client = Client::new();	
	let _ = client.delete(format!("http://localhost:8080/api/usuarios/{id}").as_str())
		.send()
		.await;
}