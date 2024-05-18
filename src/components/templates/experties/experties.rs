use leptos::*;

use crate::components::{
	atoms::{container::SectionContainer, title::H2},
	organisms::experties_card::experties_card::ExpertiesCart,
};

pub struct ExpertiesRecord {
	pub title: String,
	pub descrpition: String,
	pub img_uri: String,
	pub img_alt: String,
}

stylance::import_style!(styles, "experties.module.scss");

#[component]
pub fn ExpertiesTemplate(#[prop()] records: Vec<ExpertiesRecord>) -> impl IntoView {
	view! {
		<SectionContainer>
			<H2>"My Experties"</H2>
			<div class=styles::container>
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
