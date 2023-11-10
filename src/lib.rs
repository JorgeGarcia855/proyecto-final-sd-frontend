mod components;

use leptos::*;
use leptos_router::*;

use crate::components::pages::{home::Home, login::Login, clientes::Clientes, productos::Productos, reportes::Reportes, usuarios::Usuarios, ventas::Ventas, proveedores::Proveedores};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes> 
                <Route path="/" view=Login/>
                <Route path="/home" view=Home/>
                <Route path="/clientes" view=Clientes/>
                <Route path="/proveedores" view=Proveedores/>
                <Route path="/productos" view=Productos/>
                <Route path="/reportes" view=Reportes/>
                <Route path="/usuarios" view=Usuarios/>
                <Route path="/ventas" view=Ventas/>
            </Routes>
        </Router>
    }
}

