use leptos::*;

use crate::components::atoms::{
	container::SectionContainer,
	photo::{AspectType, Photo},
	title::H2,
};

pub struct PhotoDetails {
	pub src: String,
	pub aspect: AspectType,
}

#[component]
pub fn PhotoGalleryTemplate(records: Vec<PhotoDetails>) -> impl IntoView {
	view! {
		<SectionContainer >
			<H2>"my favorite captures 📷"</H2>
			<div
				class="
					grid
					grid-flow-row-dense
					gap-4
					w-full

					md:grid-cols-2
					lg:grid-cols-3
				"
			>
				{records
					.into_iter()
					.map(|record| {
					view! {
						<Photo src={record.src} aspect={record.aspect}/>
					}
				}).collect::<Vec<_>>()}
			</div>
		</SectionContainer>
	}
}
