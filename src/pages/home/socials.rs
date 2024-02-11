use icondata as i;
use icondata::Icon;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Socials() -> impl IntoView {
	view! {
		<div class="grid grid-rows-subgrid row-span-6">
			<h2 class="text-4xl uppercase my-2">"Socials 🙋"</h2>
			<div class="grid grid-rows-subgrid row-span-5 auto-rows-fr gap-2">
				<SocialMedia
					icon=i::BsLinkedin
					url="https://www.linkedin.com/in/srinesh-nisala".to_string()
					description="I share my work here".to_string()
				/>
				<SocialMedia
					icon=i::BsGithub
					url="https://github.com/s1n7ax".to_string()
					description="My opensource projects".to_string()
				/>
				<SocialMedia
					icon=i::BsYoutube
					url="https://www.youtube.com/@s1n7ax".to_string()
					description="I make videos about Linux, Docker, Neovim etc...".to_string()
				/>
				<SocialMedia
					icon=i::BsFacebook
					url="https://web.facebook.com/s1n7ax".to_string()
					description="If you are into top notch memes, find me here".to_string()
				/>
			</div>
		</div>
	}
}

#[component]
fn SocialMedia(icon: Icon, url: String, description: String) -> impl IntoView {
	view! {
		<a
			href=url
			target="_blank"
			class="grid grid-flow-col auto-cols-[auto_1fr] gap-x-4 group items-center"
		>
			<Icon
				icon=icon
				height="100%"
				width="100%"
				class="aspect-square
				w-6
				text-gray-600
				group-hover:text-gray-800
				transition
				ease-in-out
				duration-200"
			/>
			<span class="text-gray-600 group-hover:text-gray-800">{description}</span>
		</a>
	}
}
