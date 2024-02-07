use leptos::*;

use super::project_card::ProjectCard;

struct ProjectDetails {
	name: String,
	short_description: String,
	long_description: String,
	video_uri: String,
	links: Vec<String>,
}

#[component]
pub fn Projects() -> impl IntoView {
	let records: [ProjectDetails; 3] = [
		ProjectDetails {
			name: "Computer Vision Automation Tool".to_string(),
			short_description: "CVAT leverages computer vision for UI automation".to_string(),
			long_description: "CVAT employs an object detection model combined with an Optical
				Character Recognition model to interpret the user interface into a
				human-understandable representation, allowing automation engineers to
				utilize a customized and simplified traversal path for capturing and
				executing actions on UI elements."
				.to_string(),
			video_uri: "images/me_01.webp".to_string(),
			links: vec![
				"https://youtube.com/playlist?list=PL0EgBggsoPCm_SgDWaDDOEk7tD1dsmM8K&si=xF3YzpO-UjM-mHd9"
					.to_string(),
			],
		},
		ProjectDetails {
			name: "Java for Neovim".to_string(),
			short_description: "Java for Neovim provides IDE features such as auto-completion,
				diagnostics, debugging for Neovim editor"
				.to_string(),
			long_description:
				"This plugin combines Neovim's swift editing style with robust Java IDE features,
				utilizing the Language Server Protocol and Debug Adapter Protocol. It enables
				seamless auto-completion, diagnostics, and supports the execution, debugging,
				and testing of JUnit tests for Java applications developed with Gradle, Maven,
				or Eclipse."
					.to_string(),
			video_uri: "images/me_01.webp".to_string(),
			links: vec![
				"https://github.com/nvim-java/nvim-java".to_string(),
				"https://youtu.be/CXv0WUX_E_Q".to_string(),
			],
		},
		ProjectDetails {
			name: "Open Unicode Converter".to_string(),
			short_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			long_description:
				"The Open Unicode converter is a highly customizable tool, making the addition
				of new languages as simple as incorporating a new JSON entry. It features an
				Angular UI for converting Sinhala language with various styling options,
				including bold formatting, headers, italicized text, bullet points, and more."
					.to_string(),
			video_uri: "images/me_01.webp".to_string(),
			links: vec![
				"https://github.com/s1n7ax/open-unicode-converter".to_string(),
				"https://www.sinhalaunicode.org".to_string(),
			],
		},
	];

	view! {
		<>
			<div class="grid grid-cols-6 gap-4 place-content-center">
				<h2 class="col-start-2 text-4xl uppercase mb-2 mt-7 font-mono">"My projects"</h2>

				{records
					.into_iter()
					.map(|record| {
						view! {
							<div class="row-start-2 row-span-4 col-span-2 grid place-items-center">
								<ProjectCard
									name=record.name
									short_description=record.short_description
									long_description=record.long_description
									video_uri=record.video_uri
									links=record.links
								/>
							</div>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</>
	}
}
