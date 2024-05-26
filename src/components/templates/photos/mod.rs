use leptos::*;

use crate::components::{
	atoms::{container::SectionContainer, photo::AspectType, title::H2},
	organisms::carousel::Carousel,
};

stylance::import_style!(styles, "photos.module.scss");

#[derive(Clone)]
pub struct PhotoDetails {
	pub src: String,
	pub aspect: AspectType,
}

#[component]
pub fn PhotoGalleryTemplate(records: Vec<PhotoDetails>) -> impl IntoView {
	view! {
		<SectionContainer>
			<H2 class=styles::title>"my favorite captures 📷"</H2>
			<Carousel images=records />
		</SectionContainer>
	}
}
