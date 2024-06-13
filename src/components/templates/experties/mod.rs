use leptos::*;
use leptos_use::use_intersection_observer;

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

stylance::import_style!(styles, "experties.module.scss");

#[component]
pub fn ExpertiesTemplate(#[prop()] records: Vec<ExpertiesRecord>) -> impl IntoView {
	let el = create_node_ref::<html::Div>();
	let (is_visible, set_visible) = create_signal(false);

	use_intersection_observer(el, move |entries, _| {
		set_visible.set(entries[0].is_intersecting());
	});

	let classes = move || {
		if is_visible.get_untracked() {
			format!("{} {}", styles::container, styles::container_visible)
		} else {
			is_visible.get();
			styles::container.to_string()
		}
	};

	view! {
		<SectionContainer>
			<div class=styles::title_container>
				<H2>"My Experties"</H2>
			</div>
			<div
				class=move|| classes()
				node_ref=el
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
