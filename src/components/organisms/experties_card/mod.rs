use leptos::*;
use leptos_image::Image;

use crate::components::atoms::title::H3;

stylance::import_style!(styles, "experties_card.module.scss");

#[component]
pub fn ExpertiesCart(
	title: String,
	description: String,
	img_uri: String,
	img_alt: String,
) -> impl IntoView {
	view! {
		<article class=styles::container>
			<Image
				width=100
				height=100
				class=styles::image
				src=img_uri
				quality=100
				alt=img_alt
			/>
			<H3>{title}</H3>
			<p>{description}</p>
		</article>
	}
}
