use super::about::About;
use super::experties::Experties;
use super::intro::Intro;
use super::photos::PhotosGallery;
// use super::projects::Projects;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<div class="grid">
			<section>
				<Intro/>
			</section>
			<section>
				<About/>
			</section>
			<section>
				<PhotosGallery/>
			</section>
			<section>
				<Experties/>
			</section>
			<section>
				<div class="pb-6"></div>
			</section>
		</div>
	}
}
