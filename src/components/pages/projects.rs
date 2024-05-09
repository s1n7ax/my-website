use leptos::*;

use crate::components::templates::projects::{ProjectDetails, ProjectsTemplate};

#[component]
pub fn Projects() -> impl IntoView {
	let records = vec![
		ProjectDetails {
			name: "Computer Vision Automation Tool".to_string(),
			// short_description: "CVAT leverages computer vision for UI automation".to_string(),
			long_description: "CVAT employs an object detection model combined with an Optical
				Character Recognition model to interpret the user interface into a
				human-understandable representation, allowing automation engineers to
				utilize a customized and simplified traversal path for capturing and
				executing actions on UI elements."
				.to_string(),
			video_uri: "images/projects/cvat.png".to_string(),
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
			video_uri: "images/projects/nvim-java.png".to_string(),
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
			video_uri: "images/projects/open-unicode-converter.png".to_string(),
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
			video_uri: "images/projects/simple-message-router.png".to_string(),
			links: vec![
				"https://github.com/s1n7ax/simple-message-router".to_string(),
				"".to_string(),
			],
		},
	];

	view! {
		<ProjectsTemplate records={records} />
	}
}
