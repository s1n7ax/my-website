use super::experties::Experties;
// use super::intro::Intro;
// use super::projects::Projects;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<div class="grid">
			// <Intro/>
			<Experties/>
			<div class="pb-6"></div>
		// <Projects/>
		</div>
	}
}
