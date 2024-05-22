use leptos::*;
use leptos_image::Image;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::atoms::link::Link;

stylance::import_style!(styles, "period.module.scss");

#[component]
pub fn PeriodAt(
	location: String,
	description: String,
	start_date: String,
	end_date: String,
	url: String,
	url_label: String,
	#[prop(optional)] logo: Option<String>,
	#[prop(optional)] logo_alt: Option<String>,
) -> impl IntoView {
	let image = move || {
		logo.map(|url| {
			view! {
				<Image
					width=100
					height=100
					class=styles::image
					quality=85
					blur=true
					src=url
					alt=logo_alt.unwrap_throw()
				/>
			}
		})
	};

	view! {
		<Link
			href=url
			title=url_label.clone()
			label=url_label.clone()
			// title=format!("Link to {} website", location)
			// label=format!("Link to {} website", location)
			class=styles::link
		>
			<article class=styles::article>
				<div class=styles::text_container_with_img>
					{ image() }
					<header>
						<span>
							<span class=styles::text_bold>{description}</span>
							<span>" at "</span>
							<span class=styles::text_bold>{location}</span>
						</span>
					</header>
					<footer>
						<span class=styles::time_range>
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
