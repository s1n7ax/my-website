use leptos::*;

#[component]
pub fn Link(
	href: String,
	title: String,
	label: String,
	#[prop(optional, into)] class: Option<AttributeValue>,
	#[prop(default="_blank".to_string())] target: String,
	children: Children,
) -> impl IntoView {
	view! {
		<a
			href=href
			aria-label=label.clone()
			label=label.clone()
			target=target
			title=title
			class=class
		>
			{children()}
		</a>
	}
}
