use leptos::*;

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
				<img
					class="object-cover object-center col-span-full aspect-4/3 w-full bg-white p-4"
					class="lg:col-span-2"
					src=src
					alt=""
				/>
			}
		}

		AspectType::Portrait => {
			view! {
				<img
					class="object-cover object-center aspect-3/4 w-full bg-white p-4"
					class="lg:h-full"
					src=src
					alt=""
				/>
			}
		}

		AspectType::Square => {
			view! { <img class="object-cover object-center aspect-square w-full bg-white p-4" src=src alt=""/> }
		}
	}
}
