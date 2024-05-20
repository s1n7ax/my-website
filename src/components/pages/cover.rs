use icondata as i;
use leptos::*;

use crate::components::templates::cover::{ContactDetail, CoverTemplate};

#[component]
pub fn Cover() -> impl IntoView {
	let contacts = vec![
		ContactDetail {
			link: "tel: +94777398803".to_string(),
			link_label: "My Phone Number".to_string(),
			details: "+94 777 3988 03".to_string(),
			icon: i::BsTelephoneFill,
		},
		ContactDetail {
			link: "mailto: srineshnisala@gmail.com".to_string(),
			link_label: "My Email Address".to_string(),
			details: "srineshnisala@gmail.com".to_string(),
			icon: i::FaEnvelopeOpenTextSolid,
		},
	];

	view! {
		<CoverTemplate
			welcome="Hi!, I'm Nisala".to_string()
			designation="Senior Full-stack Developer".to_string()
			details="Hey! Welcome to my little corner of the internet!
					I'm a software whiz who loves making big business apps run smoothly.
					When I'm not glued to my computer,
					I'm out hiking, snapping pics, and having a blast playing video games.
					I'm all about free, open-source software, and I also enjoy giving
					back to the tech community.".to_string()
			contacts=contacts
		/>
	}
}
