use leptos::*;

use crate::components::{
	atoms::{container::SectionContainer, title::H2},
	organisms::experties_card::ExpertiesCart,
};

pub struct ExpertiesRecord {
	pub title: String,
	pub descrpition: String,
	pub img_uri: String,
	pub img_alt: String,
}

#[component]
pub fn ExpertiesTemplate(#[prop()] records: Vec<ExpertiesRecord>) -> impl IntoView {
	view! {
		<SectionContainer>
			<H2>"My Experties"</H2>
			<div
				class="grid
					grid-cols-1
					px-10
					gap-4

					md:w-10/12
					lg:w-10/12

					xl:grid-cols-2
					xl:w-11/12"
			>
				{records
					.into_iter()
					.map(|record| {
						view! {
							<div>
								<ExpertiesCart
									title=record.title
									description=record.descrpition
									img_uri=record.img_uri
									img_alt=record.img_alt
								/>
							</div>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</SectionContainer>
	}
}
