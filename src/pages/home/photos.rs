use leptos::*;

#[component]
pub fn PhotosGallery() -> impl IntoView {
	view! {
		<div
			class="grid justify-items-center bg-gradient-to-r from-pink-200 to-yellow-200"
			class="xl:px-24"
			class="2xl:px-32"
		>
			<h2 class="text-4xl text-center uppercase my-2 z-10">"my favorite captures 📷"</h2>
			<div
				class="grid grid-cols-1 grid-flow-row-dense px-10 gap-4 z-10"
				class="sm:grid-cols-2 md:w-10/12"
				class="md:grid-cols-2 md:w-10/12"
				class="lg:grid-cols-3 lg:w-10/12"
				class="xl:grid-cols-3 xl:w-11/12"
			>
				<Photo src="images/gallery/photo_19.jpg".to_string() aspect=AspectType::Portrait/>
				<Photo src="images/gallery/photo_21.jpg".to_string() aspect=AspectType::Landscape/>
				<Photo src="images/gallery/photo_17.jpg".to_string() aspect=AspectType::Portrait/>
				<Photo src="images/gallery/photo_7.jpg".to_string() aspect=AspectType::Portrait/>
				<Photo src="images/gallery/photo_8.jpg".to_string() aspect=AspectType::Portrait/>
				<Photo src="images/gallery/photo_23.jpg".to_string() aspect=AspectType::Portrait/>
				<Photo src="images/gallery/photo_9.jpg".to_string() aspect=AspectType::Landscape/>
				<Photo src="images/gallery/photo_26.jpg".to_string() aspect=AspectType::Portrait/>
			</div>
		</div>
	}
}

enum AspectType {
	Landscape,
	Portrait,
	Square,
}

#[component]
fn Photo(src: String, aspect: AspectType) -> impl IntoView {
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
