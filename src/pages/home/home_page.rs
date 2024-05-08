use super::intro::Intro;

use crate::components::pages::about::About;
use crate::components::pages::experties::Experties;
use crate::components::pages::photo_gallery::PhotoGallery;
use crate::components::pages::projects::Projects;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<div class="grid">
			<section>
				<Intro/>
			</section>
			<section>
				<Experties/>
			</section>
			<section>
				<Projects/>
			</section>
			<section>
				<PhotoGallery/>
			</section>
			<section>
				<About />
			</section>
		</div>
	}
}
