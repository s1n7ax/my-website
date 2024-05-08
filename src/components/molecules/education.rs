use leptos::*;

#[component]
pub fn Education(
	course: String,
	institute: String,
	start_date: String,
	end_date: String,
	url: String,
) -> impl IntoView {
	view! {
		<a
			href=url
			target="_blank"
			class="grid
			grid-flow-col
			grid-cols-[auto_1fr]
			gap-x-4
			group"
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
		</a>
	}
}
