use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::{login::Login, home::Home, clientes::Clientes, productos::Productos, proveedores::Proveedores, reportes::Reportes, usuarios::Usuarios, ventas::Ventas};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/home")]
    Home,
    #[at("/clientes")]
    Clientes,
    #[at("/productos")]
    Productos,
    #[at("/proveedores")]
    Proveedores,
    #[at("/reportes")]
    Reportes,
    #[at("/usuarios")]
    Usuarios,
    #[at("/ventas")]
    Ventas,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Login => html! { <Login/> },
        Route::Home => html! { <Home/> },
        Route::Clientes => html! { <Clientes/> },
        Route::Productos => html! { <Productos/> },
        Route::Proveedores => html! { <Proveedores/> },
        Route::Reportes => html! { <Reportes/> },
        Route::Usuarios => html! { <Usuarios/> },
        Route::Ventas => html! { <Ventas/> },
    }
}
