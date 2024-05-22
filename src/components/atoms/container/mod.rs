use leptos::*;

stylance::import_style!(styles, "container.module.scss");

#[component]
pub fn SectionContainer(
	children: Children,
	#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
	view! {
		<div
			class=styles::selection_container
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
	#[prop(optional, into)] class: Option<AttributeValue>,
	children: Children,
	label: String,
) -> impl IntoView {
	view! {
		<main
			aria-label=label
			class=styles::main
			style=move || if center {"justify-items: center"} else {""}
			class=class
		>
			{children()}
		</main>
	}
}

#[component]
pub fn Section(
	#[prop(default = true)] center: bool,
	#[prop(optional, into)] class: Option<AttributeValue>,
	children: Children,
	#[prop(default="region".to_string())] role: String,
	label: String,
) -> impl IntoView {
	view! {
		<section
			role=role
			aria-label=label
			class=styles::section
			style=move || if center {"justify-items: center"} else {""}
			class=class
		>
			{children()}
		</section>
	}
}
