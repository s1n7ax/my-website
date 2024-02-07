use leptos::*;

#[component]
pub fn ProjectCard(
	name: String,
	short_description: String,
	long_description: String,
	video_uri: String,
) -> impl IntoView {
	view! {
		<div class="
		grid
		auto-rows-min
		h-[36rem]
		w-[30rem]
		rounded-lg
		overflow-auto
		bg-zinc-700">
			<img class="aspect-video w-full" src=video_uri alt="me"/>
			<h3 class="text-white font-mono text-lg my-3 mx-3 text-xl">{name}</h3>
			<h4 class="text-gray-400 mx-3">{long_description}</h4>
		</div>
	}
}
