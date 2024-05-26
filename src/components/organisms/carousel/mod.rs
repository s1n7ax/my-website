use leptos::*;

use crate::components::templates::photos::PhotoDetails;

stylance::import_style!(styles, "carousel.module.scss");

#[component]
pub fn Carousel(images: Vec<PhotoDetails>) -> impl IntoView {
	let (focused, set_focused) = create_signal(0);

	view! {
		<div class=styles::container>
			<div class=styles::present>
				{
					images
						.clone()
						.into_iter()
						.enumerate()
						.map(|(index, image)| view! {
							<img
								class=move || if index == focused() {
									styles::focused_img
								} else {
									styles::img
								}
								src=image.src
							/>
						})
						.collect::<Vec<_>>()
				}
			</div>

			<div class=styles::thumbnail>
				{
					images
						.into_iter()
						.enumerate()
						.map(|(index, image)| view! {
							<img
								on:click=move|_| set_focused(index)
								class=move || if index == focused() {
									styles::focused_thumbnail_img
								} else {
									styles::thumbnail_img
								}
								src=image.src
							/>
						})
						.collect::<Vec<_>>()
				}
			</div>
		</div>
	}
}
