use leptos::*;
use leptos_image::Image;

use crate::components::atoms::link::Link;

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
		<Link
			href=url
			title=format!("Link to {} website", company)
			label=format!("Link to {} website", company)
			class="grid
			grid-flow-col
			grid-cols-[auto_1fr]
			gap-x-4
			group"
		>
			<article class="grid grid-flow-col gap-x-4">
				<Image
					width=100
					height=100
					class="
						object-cover aspect-square w-14
					"
					src=logo
					quality=85
					blur=true
					alt=logo_alt
				/>
				<div class="text-gray-600 group-hover:text-gray-800">
					<header>
						<span>
							<span class="font-bold">{designation}</span>
							<span>" at "</span>
							<span class="font-bold">{company}</span>
						</span>
					</header>
					<footer>
						<span class="text-sm text-gray-500">
							<span>{start_date}</span>
							<span>" - "</span>
							<span>{end_date}</span>
						</span>
					</footer>
				</div>
			</article>
		</Link>
	}
}
