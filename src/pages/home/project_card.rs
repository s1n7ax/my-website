use leptos::*;

#[component]
pub fn ProjectCard(
	name: String,
	short_description: String,
	long_description: String,
	video_uri: String,
) -> impl IntoView {
	view! {
		<div class="grid grid-cols-3 grid-rows-3 auto-rows-min aspect-square max-w-96">
			<img class="row-start-1 col-start-1 col-span-3 row-span-full" src=video_uri alt="me"/>
			<div class="
			row-start-1
			col-start-1
			col-span-full
			bg-gradient-to-b
			from-zinc-900
			to-transparent
			text-center">
				<h3 class="text-white place-self-end p-3">{name}</h3>
			</div>
			<div class="col-start-1
			row-start-3
			col-span-full
			bg-gradient-to-b
			from-transparent
			via-zinc-900
			to-zinc-900
			text-white
			text-center
			flex
			">
				<h4 class="text-white place-self-end p-3">{short_description}</h4>
			</div>

		</div>
	}
}
