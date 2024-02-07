use leptos::*;

use icondata as i;
use leptos_icons::*;

#[component]
pub fn ProjectCard(
	name: String,
	short_description: String,
	long_description: String,
	video_uri: String,
	links: Vec<String>,
) -> impl IntoView {
	view! {
		<div class="
		grid
		relative
		auto-rows-min
		w-[30rem]
		rounded-lg
		overflow-auto
		bg-zinc-700">
			<img class="aspect-video w-full" src=video_uri alt="me"/>
			<h3 class="text-white font-mono text-lg my-3 mx-3 text-xl">{name}</h3>
			<p class="text-gray-400 mx-3 h-64">{long_description}</p>
			<div class="absolute grid grid-flow-col gap-3 p-3 bottom-0 right-0">
				{links
					.into_iter()
					.map(|link| {
						let icon = if link.starts_with("https://github.com") {
							i::FaGithubAltBrands
						} else if link.starts_with("https://youtu.be")
							|| link.starts_with("https://youtube.com")
						{
							i::AiYoutubeFilled
						} else {
							i::BiWorldRegular
						};
						view! {
							<a href=link target="_blank">
								<Icon
									icon=icon
									height="100%"
									width="100%"
									class="aspect-square w-[50px] text-gray-400 hover:text-white"
								/>
							</a>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</div>
	}
}
