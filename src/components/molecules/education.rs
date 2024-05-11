use leptos::*;

use crate::components::atoms::link::Link;

#[component]
pub fn Education(
	course: String,
	institute: String,
	start_date: String,
	end_date: String,
	url: String,
	url_label: String,
) -> impl IntoView {
	view! {
		<Link
			href=url
			title=url_label.clone()
			label=url_label.clone()
			class="
				grid
				grid-flow-col
				gap-x-4
				group
			"
		>
			<div class="text-gray-600 group-hover:text-gray-800">
				<div>
					<span>
						<span class="font-bold">{course}</span>
						<span>" at "</span>
						<span class="font-bold">{institute}</span>
					</span>
				</div>
				<div>
					<span class="text-sm text-gray-500">
						<span>{start_date}</span>
						<span>" - "</span>
						<span>{end_date}</span>
					</span>
				</div>
			</div>
		</Link>
	}
}
