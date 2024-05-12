use leptos::*;
use leptos_image::Image;

#[component]
pub fn ExpertiesCart(
	title: String,
	description: String,
	img_uri: String,
	img_alt: String,
) -> impl IntoView {
	view! {
		<article class="grid auto-rows-min h-full p-5 rounded-md bg-gray-200 drop-shadow">
			<Image
				width=200
				height=200
				class="
					aspect-square
					h-24
				"
				src=img_uri
				quality=100
				alt=img_alt
			/>
			<h3 class="text-2xl mb-3 mt-4 p-0">{title}</h3>
			<p>{description}</p>
		</article>
	}
}
