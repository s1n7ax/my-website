use icondata::Icon as LeptosIconType;
use leptos::*;
use leptos_icons::Icon as LeptosIcon;

use crate::components::atoms::link::Link;

stylance::import_style!(styles, "icon.module.scss");

#[component]
pub fn Icon(link: String, label: String, icon: LeptosIconType) -> impl IntoView {
	view! {
		<Link
			href=link
			title=label.clone()
			label=label.clone()
		>
			<LeptosIcon
				icon=icon
				height="100%"
				width="100%"
				class=styles::icon
			/>
		</Link>
	}
}
