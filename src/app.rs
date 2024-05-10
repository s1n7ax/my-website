use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::home::home_page::HomePage;
use leptos::*;
use leptos_image::provide_image_context;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();
	provide_image_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/my-website.css"/>

		// sets the document title
		<Title text="Welcome to My Website"/>

		// content for this welcome page
		<Router fallback=|| {
			let mut outside_errors = Errors::default();
			outside_errors.insert_with_default_key(AppError::NotFound);
			view! { <ErrorTemplate outside_errors/> }.into_view()
		}>
			<main class="font-mono text-gray-700">
				<Routes>
					<Route path="" view=HomePage/>
				</Routes>
			</main>
		</Router>
	}
}
