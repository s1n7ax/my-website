use leptos::*;

use super::project_card::ProjectCard;

struct ProjectDetails {
	name: String,
	// short_description: String,
	long_description: String,
	video_uri: String,
	links: Vec<String>,
}

#[component]
pub fn Projects() -> impl IntoView {
	const RECORD_COUNT: usize = 4;

	let records: [ProjectDetails; RECORD_COUNT] = [
		ProjectDetails {
			name: "Computer Vision Automation Tool".to_string(),
			// short_description: "CVAT leverages computer vision for UI automation".to_string(),
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
			// short_description: "Java for Neovim provides IDE features such as auto-completion,
			// 	diagnostics, debugging for Neovim editor"
			// 	.to_string(),
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
			// short_description:
			// 	"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
			// 		.to_string(),
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
		ProjectDetails {
			name: "Simple Message Router".to_string(),
			// short_description: "".to_string(),
			long_description:
				"Simple Message Router provides a versatile solution for organizing and managing
				various communication channels such as web sockets and Chrome extension message passing.
				It acts as a centralized message router, facilitating the efficient exchange of messages
				between different components or modules within an application."
					.to_string(),
			video_uri: "images/me_01.webp".to_string(),
			links: vec![
				"https://github.com/s1n7ax/simple-message-router".to_string(),
				"".to_string(),
			],
		},
	];

	view! {
		<div class="grid auto-rows-min px-64 bg-blue-300">
			<h2 class="text-4xl uppercase mb-2 mt-7">"My Experties"</h2>
			<div class="grid grid-cols-2 gap-4">
				{records
					.into_iter()
					.map(|record| {
						view! {
							<ProjectCard
								name=record.name
								// short_description=record.short_description
								long_description=record.long_description
								video_uri=record.video_uri
								links=record.links
							/>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</div>
	}
}
