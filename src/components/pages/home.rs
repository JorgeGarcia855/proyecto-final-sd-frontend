use leptos::*;

use crate::components::organisms::{header::Header, footer::Footer};

#[component]
pub fn Home() -> impl IntoView {
	view! {
		<Header/>
		
		<Footer/>
	}
}