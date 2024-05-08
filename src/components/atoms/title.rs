use leptos::*;

#[component]
pub fn H2(children: Children) -> impl IntoView {
	view! {
		<h2 class="text-4xl
			text-center
			uppercase
			mb-2">
			{children()}
		</h2>
	}
}
