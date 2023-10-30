use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/menu")]
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
        Route::Login => html! { <h1>{ "g" }</h1>  },
        Route::Home => html! { <h1>{ "g" }</h1> },
        Route::Clientes => html! { <h1>{ "g" }</h1> },
        Route::Productos => html! { <h1>{ "g" }</h1> },
        Route::Proveedores => html! { <h1>{ "g" }</h1> },
        Route::Reportes => html! { <h1>{ "g" }</h1> },
        Route::Usuarios => html! { <h1>{ "g" }</h1> },
        Route::Ventas => html! { <h1>{ "g" }</h1> },
    }
}
