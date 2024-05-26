use leptos::*;
use leptos_image::Image;

stylance::import_style!(styles, "photo.module.scss");

#[derive(Copy, Clone)]
pub enum AspectType {
	Landscape,
	Portrait,
	Square,
}

#[component]
pub fn Photo(src: String, aspect: AspectType) -> impl IntoView {
	match aspect {
		AspectType::Landscape => {
			view! {
				<Image
					width=1024
					height=768
					class=styles::image_landscape
					src=src
					alt=""
					quality=85
					blur=true
				/>
			}
		}

		AspectType::Portrait => {
			view! {
				<Image
					width=768
					height=1024
					class=styles::image_portrait
					src=src
					alt=""
					quality=85
					blur=true
				/>
			}
		}

		AspectType::Square => {
			view! {
				<Image
					width=600
					height=600
					class=styles::image_square
					src=src
					alt=""
					quality=85
					blur=true
				/>
			}
		}
	}
}
