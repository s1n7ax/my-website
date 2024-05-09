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
        py-3

				xl:px-24
				2xl:px-32
			"
			class=class
		>
			{children()}
		</div>
	}
}
