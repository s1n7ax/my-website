use leptos::*;

use crate::components::atoms::container::{Main, Section};
use crate::components::molecules::spider::Spider;
use crate::components::pages::about::About;
use crate::components::pages::cover::Cover;
use crate::components::pages::experties::Experties;
use crate::components::pages::photo::PhotoGallery;
use crate::components::pages::projects::Projects;

stylance::import_style!(styles, "home.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
	view! {
		<div class=styles::container>
			<Main
				label="Welcome to my portfolio".to_string()
			>
				<Spider/>
				<Cover/>
			</Main>
			<Section
				label="My experties".to_string()
			>
				<Experties/>
			</Section>
			<Section
				label="Projects I have worked on".to_string()
			>
				<Projects/>
			</Section>
			<Section
				label="My favorite photos".to_string()
			>
				<PhotoGallery/>
			</Section>
			<Section
				label="My social networks, working experience and education qualification details".to_string()
			>
				<About />
			</Section>
		</div>
	}
}
