use leptos::*;

#[component]
pub fn SectionContainer(
	children: Children,
	#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
	view! {
		<div
			class="
				grid
        justify-items-center
        py-8
			"
			class=class
		>
			{children()}
		</div>
	}
}

pub enum FlowDirection {
	Row,
	Col,
	RowDense,
	ColDense,
}

#[component]
pub fn Main(
	#[prop(optional)] center: bool,
	#[prop(default=FlowDirection::Row)] direction: FlowDirection,
	#[prop(optional, into)] class: Option<AttributeValue>,
	children: Children,
	#[prop(default="region".to_string())] role: String,
	label: String,
) -> impl IntoView {
	let grid_flow_direction = move || match direction {
		FlowDirection::Row => "grid-flow-row",
		FlowDirection::Col => "grid-flow-col",
		FlowDirection::RowDense => "grid-flow-row-dense",
		FlowDirection::ColDense => "grid-flow-col-dense",
	};

	view! {
		<main
			role=role
			aria-label=label
			class="grid"
			class=move|| grid_flow_direction()
			class:justify-items-center=center
			class=class
		>
			{children()}
		</main>
	}
}

#[component]
pub fn Section(
	#[prop(default = true)] center: bool,
	#[prop(default=FlowDirection::Row)] direction: FlowDirection,
	#[prop(optional, into)] class: Option<AttributeValue>,
	children: Children,
	#[prop(default="region".to_string())] role: String,
	label: String,
) -> impl IntoView {
	let grid_flow_direction = move || match direction {
		FlowDirection::Row => "grid-flow-row",
		FlowDirection::Col => "grid-flow-col",
		FlowDirection::RowDense => "grid-flow-row-dense",
		FlowDirection::ColDense => "grid-flow-col-dense",
	};

	view! {
		<section
			role=role
			aria-label=label
			class="grid"
			class=move|| grid_flow_direction()
			class:justify-items-center=center
			class=class
		>
			{children()}
		</section>
	}
}
