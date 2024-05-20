use leptos::*;

use crate::components::{
	atoms::photo::AspectType,
	templates::photos::{PhotoDetails, PhotoGalleryTemplate},
};

#[component]
pub fn PhotoGallery() -> impl IntoView {
	let records = vec![
		PhotoDetails {
			src: "images/gallery/photo_19.jpg".to_string(),
			aspect: AspectType::Portrait,
		},
		PhotoDetails {
			src: "images/gallery/photo_21.jpg".to_string(),
			aspect: AspectType::Landscape,
		},
		PhotoDetails {
			src: "images/gallery/photo_17.jpg".to_string(),
			aspect: AspectType::Portrait,
		},
		PhotoDetails {
			src: "images/gallery/photo_7.jpg".to_string(),
			aspect: AspectType::Portrait,
		},
		PhotoDetails {
			src: "images/gallery/photo_8.jpg".to_string(),
			aspect: AspectType::Portrait,
		},
		PhotoDetails {
			src: "images/gallery/photo_23.jpg".to_string(),
			aspect: AspectType::Portrait,
		},
		PhotoDetails {
			src: "images/gallery/photo_9.jpg".to_string(),
			aspect: AspectType::Landscape,
		},
		PhotoDetails {
			src: "images/gallery/photo_26.jpg".to_string(),
			aspect: AspectType::Portrait,
		},
	];

	view! {
		<PhotoGalleryTemplate records={records} />
	}
}
