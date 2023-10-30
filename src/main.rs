use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    html! {
        
        <div class="text-3xl font-bold underline">{"hello"}</div>
        
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
