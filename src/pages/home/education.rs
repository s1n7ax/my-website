use leptos::*;

#[component]
pub fn EducationalQualification() -> impl IntoView {
	view! {
		<div class="grid grid-rows-subgrid row-span-4">
			<h2 class="text-4xl uppercase my-2">"Education 👨‍🎓"</h2>
			<div class="grid grid-rows-subgrid row-span-4 auto-rows-fr gap-2">
				<Education
					course="BSc(Hons) First Class in Computer Science and Software Engineering"
						.to_string()
					institute="University of Bedfordshire".to_string()
					start_date="September, 2020".to_string()
					end_date="July, 2021".to_string()
					url="https://www.beds.ac.uk".to_string()
				/>
				<Education
					course="Higher Diploma in Computer Based Information System".to_string()
					institute="NIBM, Sri Lanka".to_string()
					start_date="January, 2014".to_string()
					end_date="January, 2015".to_string()
					url="https://www.nibm.lk".to_string()
				/>
				<Education
					course="Hiploma in Computer System Design".to_string()
					institute="NIBM, Sri Lanka".to_string()
					start_date="January, 2015".to_string()
					end_date="January, 2016".to_string()
					url="https://www.nibm.lk".to_string()
				/>
			</div>
		</div>
	}
}

#[component]
fn Education(
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
			group
			items-center"
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
