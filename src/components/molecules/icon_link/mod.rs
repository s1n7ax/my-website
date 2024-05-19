use icondata::Icon as IconType;
use leptos::*;
use leptos_icons::Icon;

use crate::components::atoms::link::Link;

stylance::import_style!(styles, "icon_link.module.scss");

#[component]
pub fn IconLink(
	icon: IconType,
	url: String,
	url_label: String,
	description: String,
) -> impl IntoView {
	view! {
		<Link
			href=url
			label=url_label.clone()
			title=url_label.clone()
			class=styles::link
		>

		// <Link
		// 	href=url
		// 	label=url_label.clone()
		// 	title=url_label.clone()
		// 	class="grid
		// 		grid-flow-col
		// 		auto-cols-[auto_1fr]
		// 		gap-x-4
		// 		group"
		// >
			<article>
				<div class=styles::container>
					<Icon
						icon=icon
						width="100%"
						height="2rem"
						class=styles::icon
					/>
					<span>{description}</span>
				</div>
			</article>
		</Link>
	}
}
