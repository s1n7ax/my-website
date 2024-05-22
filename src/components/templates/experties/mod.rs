use leptos::*;
use leptos_use::{use_mouse_in_element, UseMouseInElementReturn};

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

	let UseMouseInElementReturn {
		x,
		y,
		source_type,
		element_x,
		element_y,
		element_position_x,
		element_position_y,
		element_width,
		element_height,
		is_outside,
		..
	} = use_mouse_in_element(el);

	view! {
		<SectionContainer>
			<H2>"My Experties"</H2>

			<div node_ref=el style="background-color: gray;">
			</div>

			<pre lang="yaml">
				x: {x}
				y: {y}
				source_type: {move || format!("{:?}", source_type())}
				element_x: {element_x}
				element_y: {element_y}
				element_position_x: {element_position_x}
				element_position_y: {element_position_y}
				element_width: {element_width}
				element_height: {element_height}
				is_outside: {is_outside}
			</pre>

			<div class=styles::container>
				{records
					.into_iter()
					.map(|record| {
						view! {
							<div>
								// <div>{ move || if is_visible() { "*****" } else { ">>>>>>" } }</div>
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
