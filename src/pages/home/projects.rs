use leptos::*;

use super::project_card::ProjectCard;

struct ProjectRecord {
	name: String,
	short_description: String,
	long_description: String,
	video_uri: String,
}

#[component]
pub fn Projects() -> impl IntoView {
	let records: [ProjectRecord; 3] = [
		ProjectRecord {
			name: "nvim-java".to_string(),
			short_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			long_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			video_uri: "images/me_01.webp".to_string(),
		},
		ProjectRecord {
			name: "nvim-java".to_string(),
			short_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			long_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			video_uri: "images/me_01.webp".to_string(),
		},
		ProjectRecord {
			name: "nvim-java".to_string(),
			short_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			long_description:
				"Java IDE features such as auto-completion, diagnostics, debugging to Neovim editor"
					.to_string(),
			video_uri: "images/me_01.webp".to_string(),
		},
	];

	view! {
		<>
			<div class="grid grid-cols-6 gap-4">
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
								/>
							</div>
						}
					})
					.collect::<Vec<_>>()}
			</div>
		</>
	}
}
