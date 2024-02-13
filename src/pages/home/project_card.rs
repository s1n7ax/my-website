use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn ProjectCard(
	name: String,
	// short_description: String,
	long_description: String,
	video_uri: String,
	links: Vec<String>,
) -> impl IntoView {
	view! {
		<div class="
		grid
		auto-rows-[auto_auto_auto_1fr]
		rounded-lg
		overflow-hidden
		bg-gray-200">
			<img class="aspect-video w-full" src=video_uri alt="me"/>
			<h3 class="text-lg my-3 mx-3 text-xl">{name}</h3>
			<p class="mx-3">{long_description}</p>
			<div class="self-end justify-self-end p-2 gap-2 grid grid-flow-col">
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
									class="aspect-square w-12 text-gray-400 hover:text-gray-800 transition ease-in-out duration-200"
								/>
							</a>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</div>
	}
}
