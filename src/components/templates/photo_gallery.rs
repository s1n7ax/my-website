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
		<SectionContainer class="bg-gradient-to-r from-pink-200 to-yellow-200">
			<H2>"my favorite captures 📷"</H2>
			<div
				class="grid grid-cols-1 grid-flow-row-dense px-10 gap-4 z-10
				md:grid-cols-2 md:w-10/12
				lg:grid-cols-3 lg:w-10/12
				xl:grid-cols-3 xl:w-11/12"
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
