use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usuarios {
    pub cedula: i64,
    pub email: String,
    pub nombre: String,
    pub password: String,
    pub usuario: String,
}


pub async fn fetch_user(id: Option<i64>) -> Option<Usuarios> {
	match id {
		Some(id) => {
			let user = reqwest::get(format!("http://localhost:8080/api/usuarios/{}", id).as_str())
				.await.unwrap()
				.json::<Usuarios>()
				.await.unwrap();
			Some(user)
		},
		None => None
	}
}

