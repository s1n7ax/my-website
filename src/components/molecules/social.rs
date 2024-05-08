// use icondata as i;
use icondata::Icon as IconType;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn SocialMediaRecord(icon: IconType, url: String, description: String) -> impl IntoView {
	view! {
		<a
			href=url
			target="_blank"
			class="grid
				grid-flow-col
				auto-cols-[auto_1fr]
				gap-x-4
				group"
		>
			<div>
				<div class="grid
					grid-flow-col
					auto-cols-[auto_1fr]
					gap-x-4
					h-1 items-center"
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
			</div>
		</a>
	}
}
