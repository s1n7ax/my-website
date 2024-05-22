use crate::components::{
	atoms::{container::SectionContainer, title::H2},
	molecules::{icon_link::IconLink, period::PeriodAt},
};
use icondata::Icon as IconType;
use leptos::*;

stylance::import_style!(styles, "about.module.scss");

#[component]
pub fn AboutTemplate(
	socials: Vec<SocialDetails>,
	work_history: Vec<PeriodDetailsWithLogo>,
	education: Vec<PeriodDetails>,
) -> impl IntoView {
	view! {
		<SectionContainer>
			<div class=styles::container>
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
		<div class=styles::about_card_container>
			<hr class=styles::hr />
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
						<IconLink
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

pub struct PeriodDetails {
	pub description: String,
	pub place: String,
	pub start_date: String,
	pub end_date: String,
	pub url: String,
	pub url_label: String,
}

pub struct PeriodDetailsWithLogo {
	pub description: String,
	pub place: String,
	pub start_date: String,
	pub end_date: String,
	pub url: String,
	pub url_label: String,
	pub logo: String,
	pub logo_alt: String,
}

#[component]
pub fn WorkHistoryTemplate(records: Vec<PeriodDetailsWithLogo>) -> impl IntoView {
	view! {
		<AboutCard title="Work 🧑‍🔧".to_string()>
			{records
				.into_iter()
				.map(|record| {
					view! {
						<PeriodAt
							description={record.description}
							location={record.place}
							start_date={record.start_date}
							end_date={record.end_date}
							url={record.url}
							url_label={record.url_label}
							logo={record.logo}
							logo_alt={record.logo_alt}
						/>
					}
				}).collect::<Vec<_>>()}
		</AboutCard>
	}
}

#[component]
pub fn EducationalQualificationTemplate(records: Vec<PeriodDetails>) -> impl IntoView {
	view! {
		<AboutCard title="Education 👨‍🎓".to_string()>
			{records
				.into_iter()
				.map(|record| {
					view! {
						<PeriodAt
							location={record.place}
							description={record.description}
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
