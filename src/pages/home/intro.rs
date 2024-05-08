use icondata as i;
use icondata::Icon;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Intro() -> impl IntoView {
	view! {
		<div
			class="grid justify-items-center bg-red-200 pb-3
			xl:grid-flow-col xl:grid-cols-2 xl:px-24
			xl:grid-flow-col xl:grid-cols-2 2xl:px-32"
		>
			<CoverPhoto/>
			<Header/>
		</div>
	}
}

#[component]
fn Header() -> impl IntoView {
	view! {
		<div
			class="grid justify-items-center gap-4 px-10"
			class="xl:col-start-1 xl:self-end xl:justify-items-start"
		>
			<h1 class="text-2xl text-center uppercase" class="xl:text-left">
				"Hi!, I'm Nisala"
			</h1>
			<h2 class="text-4xl text-center uppercase" class="xl:text-left">
				"Senior Full-stack Developer"
			</h2>
			<p class="md:px-10" class="lg:px-24" class="xl:px-0">
				"Hey! Welcome to my little corner of the internet! I'm a software whiz who
				loves making big business apps run smoothly. When I'm not glued to my computer,
				I'm out hiking, snapping pics, and having a blast playing video games.
				I'm all about free, open-source software, and I also enjoy giving back to the tech community."
			</p>
			<div
				class="grid gap-2 justify-items-center"
				class="xl:grid-flow-col xl:gap-4 xl:justify-self-end"
			>
				<Contact
					link="tel: +94777398803".to_string()
					details="+94 777 3988 03".to_string()
					icon=i::BsTelephoneFill
				/>
				<Contact
					link="mailto: srineshnisala@gmail.com".to_string()
					details="srineshnisala@gmail.com".to_string()
					icon=i::FaEnvelopeOpenTextSolid
				/>
			</div>
		</div>
	}
}

#[component]
fn Contact(link: String, details: String, icon: Icon) -> impl IntoView {
	view! {
		<a href=link target="_blank" class="grid grid-flow-col auto-cols-min gap-4">
			<Icon
				icon=icon
				height="100%"
				width="100%"
				class="aspect-square w-6 hover:text-gray-800 transition ease-in-out duration-200"
			/>
			<span class="whitespace-nowrap">{details}</span>
		</a>
	}
}

#[component]
fn CoverPhoto(#[prop(optional)] class: String) -> impl IntoView {
	view! {
		<div
			class="w-10/12 my-3
			sm:w-9/12
			md:w-8/12
			lg:w-6/12
			xl:w-full xl:col-start-2
			".to_owned() + &class
		>
			<svg
				id="visual"
				viewBox="170 100 570 400"
				xmlns="http://www.w3.org/2000/svg"
				xmlns:xlink="http://www.w3.org/1999/xlink"
				version="1.1"
				class="aspect-square"
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

				<g transform="translate(452.5223569084818 314.37079001076097)">
					<path
						d="M134.5 -237C174.6 -209.7 207.7 -174.4 232.1 -133.5C256.5 -92.7 272.3 -46.3 270.5 -1C268.8 44.3 249.6 88.7 226.7 132.1C203.7 175.6 177.1 218.1 138.7 240.1C100.3 262 50.2 263.4 0.6 262.3C-49 261.3 -98 258 -137.7 236.8C-177.4 215.6 -207.8 176.5 -229.3 134.1C-250.8 91.7 -263.4 45.8 -265.4 -1.2C-267.4 -48.2 -258.9 -96.3 -237.9 -139.8C-217 -183.2 -183.8 -221.9 -142 -248.3C-100.3 -274.6 -50.2 -288.6 -1.5 -286C47.2 -283.4 94.3 -264.2 134.5 -237"
						fill="#F79797"
					></path>
				</g>

				<g transform="translate(458.84266227862224 298.9927987124721)">
					<path
						d="M137.6 -236.6C180.2 -213.7 217.8 -180.6 241.2 -139.5C264.7 -98.3 273.8 -49.2 274.3 0.2C274.7 49.7 266.4 99.3 242.8 140.2C219.2 181 180.4 213.1 137.3 237.7C94.3 262.4 47.2 279.7 1.5 277.1C-44.1 274.4 -88.2 251.8 -127.8 225.2C-167.3 198.5 -202.3 167.7 -229.8 129.5C-257.2 91.3 -277.1 45.7 -281.2 -2.4C-285.3 -50.4 -273.6 -100.8 -246.6 -139.8C-219.6 -178.8 -177.3 -206.3 -133.7 -228.6C-90 -250.9 -45 -267.9 1.2 -270.1C47.5 -272.3 95 -259.5 137.6 -236.6"
						fill="#FF7373"
					></path>
				</g>

				<g transform="translate(454.5655440076682 304.8968139317783)">
					<path
						d="M137.5 -239.6C175.7 -216.2 202.5 -174.3 225.2 -131.2C247.8 -88.2 266.4 -44.1 269.2 1.6C272 47.3 259 94.7 236.7 138.3C214.4 182 182.8 222 141.8 242.3C100.8 262.6 50.4 263.3 0.4 262.7C-49.7 262 -99.3 260.1 -141.6 240.5C-183.9 220.9 -218.8 183.7 -240.7 140.7C-262.5 97.7 -271.2 48.8 -268.1 1.8C-264.9 -45.2 -249.8 -90.3 -227.7 -132.9C-205.6 -175.5 -176.6 -215.5 -137.4 -238.4C-98.3 -261.3 -49.2 -267.2 0.2 -267.6C49.7 -268 99.3 -263.1 137.5 -239.6"
						fill="#FC5353"
					></path>
				</g>
				<g transform="translate(461.2435766118532 299.62190761821614)">
					// <g transform="translate(461.2435766118532 299.62190761821614)">
					<path
						fill="url(#imgpattern)"
						d="M147.7 -233.2C188.1 -204 215 -157.2 232.9 -109C250.7 -60.8 259.5 -11.1 252.5 36.2C245.6 83.6 222.9 128.5 191.8 168.3C160.7 208.1 121 242.7 74.3 259.2C27.6 275.7 -26.2 274.1 -77.4 260.5C-128.7 246.9 -177.3 221.4 -211.7 182.8C-246 144.2 -266 92.7 -273.8 39.3C-281.7 -14.1 -277.3 -69.4 -253.3 -113C-229.2 -156.7 -185.6 -188.8 -140.1 -215.7C-94.6 -242.6 -47.3 -264.3 3.2 -269.3C53.7 -274.2 107.4 -262.5 147.7 -233.2"
					></path>
				</g>
			</svg>
		</div>
	}
}
