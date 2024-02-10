use icondata as i;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Intro() -> impl IntoView {
	view! {
		<div class="grid auto-cols-fr grid-flow-col px-64 bg-red-300">
			<div class="grid content-end px-10 py-24">
				<div>
					<h1 class="text-2xl uppercase py-2">"Hi!, I'm Nisala"</h1>
					<h2 class="text-4xl uppercase py-2">"Senior Full-stack Developer"</h2>
					<p>
						"
						Welcome to my digital realm! I'm a senior software engineer
						dedicated to enhancing Enterprise web applications.
						"
					</p>
					<div class="grid grid-flow-col gap-4 justify-end p-2">
						<a
							href="tel: +94777398803"
							target="_blank"
							class="grid grid-flow-col gap-1"
						>
							<Icon
								icon=i::FaPhoneSolid
								height="100%"
								width="100%"
								class="aspect-square w-4 hover:text-gray-800 transition ease-in-out duration-200"
							/>
							<span>+94-777-3988-03</span>
						</a>
						<a
							href="mailto: srineshnisala@gmail.com"
							target="_blank"
							class="grid grid-flow-col gap-1"
						>
							<Icon
								icon=i::BsEnvelopePaperFill
								height="100%"
								width="100%"
								class="aspect-square w-4 hover:text-gray-800 transition ease-in-out duration-200"
							/>
							<span>"srineshnisala@gmail.com"</span>
						</a>

					</div>
				</div>
			</div>
			<svg
				id="visual"
				viewBox="150 100 600 400"
				xmlns="http://www.w3.org/2000/svg"
				xmlns:xlink="http://www.w3.org/1999/xlink"
				version="1.1"
				class="aspect-square w-[40rem]"
			>
				<defs>
					<pattern
						id="imgpattern"
						width="1"
						height="1"
						viewBox="0 0 100 100"
						preserveAspectRatio="xMidYMid slice"
					>
						<image width="100" height="100" href="images/me_02.webp"></image>
					</pattern>
				</defs>
				<g transform="translate(461.2435766118532 299.62190761821614)">
					<path
						fill="url(#imgpattern)"
						d="M147.7 -233.2C188.1 -204 215 -157.2 232.9 -109C250.7 -60.8 259.5 -11.1 252.5 36.2C245.6 83.6 222.9 128.5 191.8 168.3C160.7 208.1 121 242.7 74.3 259.2C27.6 275.7 -26.2 274.1 -77.4 260.5C-128.7 246.9 -177.3 221.4 -211.7 182.8C-246 144.2 -266 92.7 -273.8 39.3C-281.7 -14.1 -277.3 -69.4 -253.3 -113C-229.2 -156.7 -185.6 -188.8 -140.1 -215.7C-94.6 -242.6 -47.3 -264.3 3.2 -269.3C53.7 -274.2 107.4 -262.5 147.7 -233.2"
						fill="#BB004B"
					></path>
				</g>
			</svg>

		</div>
	}
}
