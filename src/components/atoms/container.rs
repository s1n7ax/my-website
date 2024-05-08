use leptos::*;

#[component]
pub fn SectionContainer(children: Children) -> impl IntoView {
	view! {
		<div
			class="grid justify-items-center py-3
			xl:px-24
			2xl:px-32"
		>
			{children()}
		</div>
	}
}
