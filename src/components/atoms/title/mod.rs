use leptos::*;

stylance::import_style!(styles, "title.module.scss");

#[component]
pub fn H1(
	children: Children,
	#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
	view! {
		<h1 class=styles::header_1 class=class>
			{children()}
		</h1>
	}
}

#[component]
pub fn H2(
	children: Children,
	#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
	view! {
		<h2 class=styles::header_2 class=class>
			{children()}
		</h2>
	}
}

#[component]
pub fn H3(
	children: Children,
	#[prop(optional, into)] class: Option<AttributeValue>,
) -> impl IntoView {
	view! {
		<h3 class=styles::header_3 class=class>
			{children()}
		</h3>
	}
}
