use leptos::*;

#[component]
pub fn Intro() -> impl IntoView {
	view! {
		<>
			<div class="col-start-2 col-span-2 grid grid-cols-3 grid-rows-4">
				<div class="row-start-2 row-span-3 col-span-3 m-4 flex flex-col gap-1">
					<h1 class="text-2xl uppercase py-2 font-mono">"Hi, I'm Nisala"</h1>
					<h2 class="text-4xl uppercase py-2 font-mono">"Full-stack Developer"</h2>
					<p class="font-mono">
						"
						Welcome to my digital realm! I'm a senior software engineer
						dedicated to enhancing Enterprise web applications.
						"
					</p>
					<button class="
					border-2
					border-gray-900
					hover:bg-gray-800
					hover:text-white
					rounded-full
					py-3
					px-4
					uppercase
					text-xs
					font-bold
					cursor-pointer
					tracking-wide">"Contact Me"</button>
				</div>
			</div>
			<div class="col-span-2">
				<img src="images/me_01.webp" alt="me" width="500"/>
			</div>
		</>
	}
}
