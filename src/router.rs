use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::{login::Login, home::Home};

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
        Route::Home => html! { <Home/>},
        Route::Clientes => html! { <h1>{ "clientes" }</h1> },
        Route::Productos => html! { <h1>{ "productos" }</h1> },
        Route::Proveedores => html! { <h1>{ "proveedores" }</h1> },
        Route::Reportes => html! { <h1>{ "reportes" }</h1> },
        Route::Usuarios => html! { <h1>{ "usuarios" }</h1> },
        Route::Ventas => html! { <h1>{ "ventas" }</h1> },
    }
}
