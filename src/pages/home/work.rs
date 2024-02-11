use leptos::*;

#[component]
pub fn WorkInfo() -> impl IntoView {
	view! {
		<div class="grid grid-rows-subgrid row-span-6">
			<h2 class="text-4xl uppercase my-2">"Work 🧑‍🔧"</h2>
			<div class="grid grid-rows-subgrid row-span-4 auto-rows-fr gap-2">
				<Work
					designation="Senior Software Engineer".to_string()
					company="Orli Tech".to_string()
					start_date="February 15, 2024".to_string()
					end_date="Present".to_string()
					url="https://orli.tech".to_string()
					logo="images/logos/orli_tech.webp".to_string()
					logo_alt="Orli Tech logo".to_string()
				/>
				<Work
					designation="Software Engineer".to_string()
					company="MC Medisoft".to_string()
					start_date="March 7, 2021".to_string()
					end_date="February 15, 2024".to_string()
					url="https://mcmedisoft.com".to_string()
					logo="images/logos/mc_medisoft.jpg".to_string()
					logo_alt="MC Medisoft logo".to_string()
				/>
				<Work
					designation="Software Engineer".to_string()
					company="Infor Sri Lanka".to_string()
					start_date="September 26, 2017".to_string()
					end_date="March 5, 2021".to_string()
					url="https://www.infor.com".to_string()
					logo="images/logos/infor.png".to_string()
					logo_alt="Infor logo".to_string()
				/>
				<Work
					designation="Automation Associate Specialist".to_string()
					company="Virtusa".to_string()
					start_date="February 16, 2016".to_string()
					end_date="September 4, 2017".to_string()
					url="https://www.virtusa.com".to_string()
					logo="images/logos/virtusa.jpg".to_string()
					logo_alt="Virtusa logo".to_string()
				/>
			</div>
		</div>
	}
}

#[component]
fn Work(
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
			group
			items-center"
		>
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
		</a>
	}
}
