use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"HOME"}</h1>
            <Link<Route> to={Route::Login}>{"go back"}</Link<Route>>
        </div>
    }
}