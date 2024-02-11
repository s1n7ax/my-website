use leptos::*;

#[component]
pub fn PhotosGallery() -> impl IntoView {
	view! {
		<div class="grid auto-rows-min px-64 py-4 bg-gradient-to-bl from-slate-100 to-gray-300">
			<h2 class="text-4xl uppercase my-2">"my favorite captures 📷"</h2>
			<div class="grid
			grid-cols-12
			auto-rows-auto
			grid-flow-row-dense
			gap-4
			aspect-square
			auto-cols-fr
			auto-rows-fr
			w-full">
				<img
					class="object-cover object-top"
					class="col-span-4 row-span-6"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_19.jpg"
					alt=""
				/>
				<img
					class="object-cover object-top"
					class="col-span-6 row-span-4"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_21.jpg"
					alt=""
				/>
				<img
					class="object-cover object-top"
					class="col-span-2 row-span-3"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_24.jpg"
					alt=""
				/>
				<img
					class="object-cover object-top"
					class="col-span-2 row-span-3"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_15.jpg"
					alt=""
				/>
				<img
					class="object-cover object-center"
					class="col-span-4 row-span-3"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_3.jpg"
					alt=""
				/>
				<img
					class="object-cover object-top"
					class="col-span-3 row-span-4"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_23.jpg"
					alt=""
				/>
				<img
					class="object-cover object-top"
					class="col-span-6 row-span-4"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_9.jpg"
					alt=""
				/>
				<img
					class="object-cover object-center"
					class="col-span-3 row-span-3"
					class="object-cover w-full h-full shadow-xl bg-gray-200 p-3"
					src="images/gallery/photo_26.jpg"
					alt=""
				/>
			</div>
		</div>
	}
}
