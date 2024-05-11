use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_image::Image;

use crate::components::atoms::link::Link;

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
			<Image
				width=1280
				height=960
				class="
					aspect-video
					w-full
				"
				src=video_uri
				quality=85
				blur=true
				alt=""
			/>
			<h3 class="text-lg my-3 mx-3 text-xl">{name}</h3>
			<p class="mx-3">{long_description}</p>
			<div class="self-end justify-self-end p-2 gap-2 grid grid-flow-col">
				{links
					.into_iter()
					.map(|link| {
						let icon_details =
						if link.starts_with("https://github.com") { (i::FaGithubAltBrands, "Link to Github Project".to_string()) }
						else if link.starts_with("https://youtube.com") { (i::AiYoutubeFilled, "Link to Youtube Videos".to_string()) }
						else { (i::BiWorldRegular, "Link to website".to_string()) };

						view! {
							<Link
								href=link
								title=icon_details.1.clone()
								label=icon_details.1.clone()
							>
								<Icon
									icon=icon_details.0
									height="100%"
									width="100%"
									class="aspect-square w-12 text-gray-400 hover:text-gray-800 transition ease-in-out duration-200"
								/>
							</Link>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</div>
	}
}
