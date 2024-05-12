use crate::components::{
	atoms::{container::SectionContainer, title::H2},
	molecules::{education::Education, social::SocialMediaRecord, timerange_record::WorkRecord},
};
use icondata::Icon as IconType;
use leptos::*;

#[component]
pub fn AboutTemplate(
	socials: Vec<SocialDetails>,
	work_history: Vec<WorkDetails>,
	education: Vec<CourseDetails>,
) -> impl IntoView {
	view! {
		<SectionContainer>
			<div class="
				grid
				grid-flow-row
				gap-4

				xl:gap-0
				xl:auto-rows-fr
				xl:grid-rows-[auto]
				xl:grid-flow-col
				xl:grid-cols-3
			">
				<SocialMediaTemplate records={socials} />
				<WorkHistoryTemplate records={work_history} />
				<EducationalQualificationTemplate records={education} />
			</div>
		</SectionContainer>
	}
}

#[component]
fn AboutCard(title: String, children: Children) -> impl IntoView {
	view! {
		<div class="
			grid
			gap-y-3
			xl:grid-rows-subgrid
			xl:row-span-5
		">
			<hr class="xl:hidden h-px my-1 bg-gray-200 border-0 dark:bg-gray-700" />
			<H2>{title}</H2>

			{children()}
		</div>
	}
}

pub struct SocialDetails {
	pub icon: IconType,
	pub url: String,
	pub url_label: String,
	pub description: String,
}

#[component]
pub fn SocialMediaTemplate(records: Vec<SocialDetails>) -> impl IntoView {
	view! {
		<AboutCard title="Socials 🙋".to_string()>
			{records
				.into_iter()
				.map(|record| {
					view! {
						<SocialMediaRecord
							icon={record.icon}
							url={record.url}
							url_label={record.url_label}
							description={record.description}
						/>
					}
				}).collect::<Vec<_>>()}
		</AboutCard>
	}
}

pub struct WorkDetails {
	pub designation: String,
	pub company: String,
	pub start_date: String,
	pub end_date: String,
	pub url: String,
	pub logo: String,
	pub logo_alt: String,
}

#[component]
pub fn WorkHistoryTemplate(records: Vec<WorkDetails>) -> impl IntoView {
	view! {
		<AboutCard title="Work 🧑‍🔧".to_string()>
			{records
				.into_iter()
				.map(|record| {
					view! {
						<WorkRecord
							designation={record.designation}
							company={record.company}
							start_date={record.start_date}
							end_date={record.end_date}
							url={record.url}
							logo={record.logo}
							logo_alt={record.logo_alt}
						/>
					}
				}).collect::<Vec<_>>()}
		</AboutCard>
	}
}

pub struct CourseDetails {
	pub course: String,
	pub institute: String,
	pub start_date: String,
	pub end_date: String,
	pub url: String,
	pub url_label: String,
}

#[component]
pub fn EducationalQualificationTemplate(records: Vec<CourseDetails>) -> impl IntoView {
	view! {
		<AboutCard title="Education 👨‍🎓".to_string()>
			{records
				.into_iter()
				.map(|record| {
					view! {
						<Education
							course={record.course}
							institute={record.institute}
							start_date={record.start_date}
							end_date={record.end_date}
							url={record.url}
							url_label={record.url_label}
						/>
					}
				}).collect::<Vec<_>>()}
		</AboutCard>
	}
}
