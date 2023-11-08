use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clientes {
    pub cedula: i64,
    pub direccion: String,
    pub email: String,
    pub nombre: String,
    pub telefono: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Usuarios {
    pub cedula: i64,
    pub email: String,
    pub nombre: String,
    pub password: String,
    pub usuario: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Productos {
    pub codigo: i64,
    pub nit_proveedor: i64,
    pub iva_compra: f64,
    pub nombre_producto: String,
    pub precio_compra: f64,
    pub precio_venta: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Proveedores {
    pub nit: i64,
   	pub ciudad: String,
    pub direccion: String,
    pub nombre: String,
    pub telefono: String,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]

pub struct Ventas {
    pub codigo: i64,
    pub cedula_cliente: i64,
    pub cedula_usuario: i64,
    pub iva_venta: f64,
    pub total_venta: f64,
    pub valor_venta: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DetalleVentas {
    pub codigo: i64,
    pub codigo_producto: i64,
    pub codigo_venta: i64,
    pub cantidad_producto: i32,
    pub valor_total: f64,
    pub valor_venta: f64,
    pub valor_iva: f64,
}
