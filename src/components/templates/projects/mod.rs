use crate::components::{
	atoms::{container::SectionContainer, title::H2},
	organisms::project_card::ProjectCard,
};
use leptos::*;

stylance::import_style!(styles, "projects.module.scss");

pub struct ProjectDetails {
	pub name: String,
	// pub short_description: String,
	pub long_description: String,
	pub video_uri: String,
	pub links: Vec<String>,
}

#[component]
pub fn ProjectsTemplate(records: Vec<ProjectDetails>) -> impl IntoView {
	const RECORD_COUNT: usize = 4;

	view! {
		<SectionContainer>
			<H2>"Some Fun Projects"</H2>
			<div class=styles::container>
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
