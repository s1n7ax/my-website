use super::experties::Experties;
use super::intro::Intro;
use super::projects::Projects;

use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<div class="grid grid-cols-4 auto-rows-min w-full">
			<div class="col-span-4 grid grid-cols-6 bg-emerald-500">
				<Intro/>
			</div>
			<div class="col-span-4">
				<Experties/>
			</div>
			<div class="col-span-4">
				<Projects/>
			</div>
			<div class="col-span-4">"4"</div>
		</div>
	}
}
