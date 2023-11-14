use leptos::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clientes {
    pub cedula: i64,
    pub direccion: String,
    pub email: String,
    pub nombre: String,
    pub telefono: String,
}

pub async fn fetch_clientes() -> Option<Vec<Clientes>> {
	let fetch: Vec<Clientes> = reqwest::get("http://localhost:8080/api/clientes/")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
	Some(fetch)
}