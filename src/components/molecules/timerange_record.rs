use leptos::*;

#[component]
pub fn WorkRecord(
	designation: String,
	company: String,
	start_date: String,
	end_date: String,
	url: String,
	logo: String,
	logo_alt: String,
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
			<div class="grid grid-flow-col gap-x-4">
				<img class="object-cover aspect-square w-14" src=logo alt=logo_alt/>
				<div class="text-gray-600 group-hover:text-gray-800">
					<div>
						<span>
							<span class="font-bold">{designation}</span>
							<span>" at "</span>
							<span class="font-bold">{company}</span>
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
			</div>
		</a>
	}
}
