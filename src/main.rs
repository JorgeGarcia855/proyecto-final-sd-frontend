use std::panic;

use leptos::*;
use proyecto_final_sd_frontend::App;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|| view! { <App/> })
} 