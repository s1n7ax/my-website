use icondata as i;
use leptos::*;

use crate::components::templates::about::{
	AboutTemplate, CourseDetails, SocialDetails, WorkDetails,
};

#[component]
pub fn About() -> impl IntoView {
	let work_history = vec![
		WorkDetails {
			designation: "Senior Software Engineer".to_string(),
			company: "Orli Tech".to_string(),
			start_date: "February 15, 2024".to_string(),
			end_date: "Present".to_string(),
			url: "https://orli.tech".to_string(),
			logo: "images/logos/orli_tech.webp".to_string(),
			logo_alt: "Orli Tech logo".to_string(),
		},
		WorkDetails {
			designation: "Software Engineer".to_string(),
			company: "MC Medisoft".to_string(),
			start_date: "March 7, 2021".to_string(),
			end_date: "February 15, 2024".to_string(),
			url: "https://mcmedisoft.com".to_string(),
			logo: "images/logos/mc_medisoft.jpg".to_string(),
			logo_alt: "MC Medisoft logo".to_string(),
		},
		WorkDetails {
			designation: "Software Engineer".to_string(),
			company: "Infor Sri Lanka".to_string(),
			start_date: "September 26, 2017".to_string(),
			end_date: "March 5, 2021".to_string(),
			url: "https://www.infor.com".to_string(),
			logo: "images/logos/infor.png".to_string(),
			logo_alt: "Infor logo".to_string(),
		},
		WorkDetails {
			designation: "Automation Associate Specialist".to_string(),
			company: "Virtusa".to_string(),
			start_date: "February 16, 2016".to_string(),
			end_date: "September 4, 2017".to_string(),
			url: "https://www.virtusa.com".to_string(),
			logo: "images/logos/virtusa.jpg".to_string(),
			logo_alt: "Virtusa logo".to_string(),
		},
	];

	let socials = vec![
		SocialDetails {
			icon: i::BsLinkedin,
			url: "https://www.linkedin.com/in/srinesh-nisala".to_string(),
			description: "I share my work here".to_string(),
		},
		SocialDetails {
			icon: i::BsGithub,
			url: "https://github.com/s1n7ax".to_string(),
			description: "My opensource projects".to_string(),
		},
		SocialDetails {
			icon: i::BsYoutube,
			url: "https://www.youtube.com/@s1n7ax".to_string(),
			description: "I make videos about Linux, Docker, Neovim etc...".to_string(),
		},
		SocialDetails {
			icon: i::BsFacebook,
			url: "https://web.facebook.com/s1n7ax".to_string(),
			description: "If you are into top notch memes, find me here".to_string(),
		},
	];

	let education = vec![
		CourseDetails {
			course: "BSc(Hons) First Class in Computer Science and Software Engineering"
				.to_string(),
			institute: "University of Bedfordshire".to_string(),
			start_date: "September, 2020".to_string(),
			end_date: "July, 2021".to_string(),
			url: "https://www.beds.ac.uk".to_string(),
		},
		CourseDetails {
			course: "Higher Diploma in Computer Based Information System".to_string(),
			institute: "NIBM, Sri Lanka".to_string(),
			start_date: "January, 2014".to_string(),
			end_date: "January, 2015".to_string(),
			url: "https://www.nibm.lk".to_string(),
		},
		CourseDetails {
			course: "Hiploma in Computer System Design".to_string(),
			institute: "NIBM, Sri Lanka".to_string(),
			start_date: "January, 2015".to_string(),
			end_date: "January, 2016".to_string(),
			url: "https://www.nibm.lk".to_string(),
		},
	];

	view! {
		<AboutTemplate
			socials={socials}
			work_history={work_history}
			education={education}
		/>
	}
}
