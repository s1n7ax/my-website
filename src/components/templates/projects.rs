use leptos::*;

use crate::components::{
	atoms::{container::SectionContainer, title::H2},
	organisms::project_card::ProjectCard,
};

pub struct ProjectDetails {
	pub name: String,
	// pub short_description: String,
	pub long_description: String,
	pub video_uri: String,
	pub links: Vec<String>,
}

#[component]
pub fn ProjectsTemplate(#[prop()] records: [ProjectDetails; 4]) -> impl IntoView {
	const RECORD_COUNT: usize = 4;

	view! {
		<SectionContainer>
			<H2>"Some Fun Projects"</H2>
			<div
				class="grid grid-cols-1 px-10 gap-4
				md:grid-cols-2 md:w-10/12
				lg:w-10/12
				xl:grid-cols-2 xl:w-11/12"
			>
				{records
					.into_iter()
					.map(|record| {
						view! {
							<ProjectCard
								name=record.name
								// short_description=record.short_description
								long_description=record.long_description
								video_uri=record.video_uri
								links=record.links
							/>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</SectionContainer>
	}
}
