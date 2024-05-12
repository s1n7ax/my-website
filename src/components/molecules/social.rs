// use icondata as i;
use icondata::Icon as IconType;
use leptos::*;
use leptos_icons::Icon;

use crate::components::atoms::link::Link;

#[component]
pub fn SocialMediaRecord(
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
			class="grid
				grid-flow-col
				auto-cols-[auto_1fr]
				gap-x-4
				group"
		>
			<article>
				<div class="grid
					grid-flow-col
					auto-cols-[auto_1fr]
					gap-x-4
					items-end"
				>
					<Icon
						icon=icon
						width="100%"
						height="100%"
						class="aspect-square
						w-6
						text-gray-600
						group-hover:text-gray-800
						transition
						ease-in-out
						duration-200"
					/>
					<span class="text-gray-600 group-hover:text-gray-800">{description}</span>
				</div>
			</article>
		</Link>
	}
}
