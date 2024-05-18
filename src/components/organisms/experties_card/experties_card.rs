use leptos::*;
use leptos_image::Image;

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
			<h3 class=styles::header>{title}</h3>
			<p>{description}</p>
		</article>
	}
}
