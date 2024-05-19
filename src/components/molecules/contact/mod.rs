use icondata::Icon;
use leptos::*;
use leptos_icons::*;

use crate::components::atoms::link::Link;

#[component]
pub fn Contact(link: String, link_label: String, details: String, icon: Icon) -> impl IntoView {
	view! {
		<Link
			href=link
			title=link_label.clone()
			label=link_label.clone()
			class="grid grid-flow-col auto-cols-min gap-4"
		>
			<Icon
				icon=icon
				height="100%"
				width="100%"
				class="
					aspect-square
					w-6
					hover:text-gray-800
					transition
					ease-in-out
					duration-200
				"
			/>
			<span class="whitespace-nowrap">{details}</span>
		</Link>
	}
}
