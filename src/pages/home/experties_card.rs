use leptos::*;

#[component]
pub fn ExpertiesCart(
	title: String,
	description: String,
	img_uri: String,
	img_alt: String,
) -> impl IntoView {
	view! {
		<div class="grid auto-rows-min h-80 p-5 rounded-md bg-gray-200 drop-shadow">
			<img class="aspect-square h-24 m-0 p-0" src=img_uri alt=img_alt/>
			<h3 class="text-2xl mb-3 mt-4 p-0">{title}</h3>
			<p>{description}</p>
		</div>
	}
}
