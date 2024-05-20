use crate::components::molecules::{contact::Contact, cover_photo::CoverPhoto};
use icondata::Icon;
use leptos::*;

stylance::import_style!(styles, "cover.module.scss");

pub struct ContactDetail {
	pub link: String,
	pub link_label: String,
	pub details: String,
	pub icon: Icon,
}

#[component]
pub fn CoverTemplate(
	welcome: String,
	designation: String,
	details: String,
	contacts: Vec<ContactDetail>,
) -> impl IntoView {
	view! {
		<div class=styles::container>
			<div class=styles::cover_photo_container>
				<CoverPhoto/>
			</div>

			<div class=styles::title_container>
				<h1 class=styles::welcome>{welcome}</h1>
				<h2 class=styles::designation>{designation}</h2>
				<p class=styles::details>{details}</p>
				<div class=styles::contacts>
					{
						contacts
						.into_iter()
						.map(|contact| {
							view! {
								<Contact
									link=contact.link
									link_label=contact.link_label
									details=contact.details
									icon=contact.icon
								/>
							}
						})
						.collect::<Vec<_>>()
					}
				</div>
			</div>
		</div>
	}
}
