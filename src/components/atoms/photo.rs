use leptos::*;
use leptos_image::Image;

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
					class="
						object-cover object-center col-span-full aspect-4/3 w-full bg-white p-4
						lg:col-span-2
					"
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
					class="
						object-cover object-center aspect-3/4 w-full bg-white p-4
						lg:h-full
					"
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
					class="
						object-cover object-center aspect-square w-full bg-white p-4
					"
					src=src
					alt=""
					quality=85
					blur=true
				/>
			}
		}
	}
}
