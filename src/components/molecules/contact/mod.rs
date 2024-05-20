use icondata as i;
use leptos::*;
use leptos_icons::Icon;

use crate::components::atoms::link::Link;

stylance::import_style!(styles, "contact.module.scss");

#[component]
pub fn Contact(link: String, link_label: String, details: String, icon: i::Icon) -> impl IntoView {
	view! {
		<Link
			href=link
			title=link_label.clone()
			label=link_label.clone()
			class=styles::link
		>
			<Icon
				class=styles::icon
				icon=icon
				height="100%"
				width="100%"
			/>
			<span class=styles::details>{details}</span>
		</Link>
	}
}
