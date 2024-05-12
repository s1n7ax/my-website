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

				px-10
				md:w-10/12
				lg:w-10/12
				xl:w-11/12
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
	#[prop(optional, into)] class: Option<AttributeValue>,
	children: Children,
	#[prop(default="region".to_string())] role: String,
	label: String,
) -> impl IntoView {
	view! {
		<main
			role=role
			aria-label=label
			class="grid"
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
	#[prop(optional, into)] class: Option<AttributeValue>,
	children: Children,
	#[prop(default="region".to_string())] role: String,
	label: String,
) -> impl IntoView {
	view! {
		<section
			role=role
			aria-label=label
			class="grid"
			class:justify-items-center=center
			class=class
		>
			{children()}
		</section>
	}
}
