use icondata as i;
use leptos::*;
use leptos_image::Image;

use crate::components::atoms::icon::Icon;
use crate::components::atoms::title::H3;

stylance::import_style!(styles, "project_card.module.scss");

#[component]
pub fn ProjectCard(
	name: String,
	// short_description: String,
	long_description: String,
	video_uri: String,
	links: Vec<String>,
) -> impl IntoView {
	view! {
		<article class=styles::container>
			<header>
				<div class="
					overflow-hidden
				">
					<Image
						width=1280
						height=960
						class=styles::image
						src=video_uri
						quality=85
						blur=true
						alt={format!("Image of {} project", name)}
					/>
				</div>
				<H3>{name}</H3>
			</header>
			<p class=styles::description>{long_description}</p>
			<footer class=styles::link_container>
				{links
					.into_iter()
					.map(|link| {
						let icon_details =
						if link.starts_with("https://github.com") { (i::FaGithubAltBrands, "Link to Github Project".to_string()) }
						else if link.starts_with("https://youtube.com") { (i::AiYoutubeFilled, "Link to Youtube Videos".to_string()) }
						else { (i::BiWorldRegular, "Link to website".to_string()) };

						view! {
							<Icon
								link=link
								label=icon_details.1
								icon=icon_details.0
							/>
						}
					})
					.collect::<Vec<_>>()}
			</footer>
		</article>
	}
}
