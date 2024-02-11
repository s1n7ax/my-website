use super::education::EducationalQualification;
use super::socials::Socials;
use super::work::WorkInfo;
use leptos::*;

#[component]
pub fn About() -> impl IntoView {
	view! {
		<div class="grid grid-cols-3 auto-cols-fr px-64 py-4 bg-gradient-to-bl from-slate-100 to-gray-300">
			<Socials/>
			<WorkInfo/>
			<EducationalQualification/>
		</div>
	}
}
