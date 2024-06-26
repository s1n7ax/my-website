use leptos::*;

use crate::components::templates::experties::{ExpertiesRecord, ExpertiesTemplate};

#[component]
pub fn Experties() -> impl IntoView {
	let records = vec![
		ExpertiesRecord {
			title: "Spring Boot".to_string(),
			descrpition: "I have utilized Spring Boot, Spring Security, and Spring Cloud
						to transition an outdated monolithic system to microservices. Collaborated
						with OpenID providers like Telia for Single Sign-On and build secure
						access and sharing of sensitive files such as medical reports through the
						Spring platform."
				.to_string(),
			img_uri: "images/logos/spring_boot_logo.webp".to_string(),
			img_alt: "spring boot logo".to_string(),
		},
		ExpertiesRecord {
			title: "React".to_string(),
			descrpition: "I have gained hands-on experience with React,
						delving into functional components and exploring the latest
						concurrency APIs brought in by React 18. Additionally,
						I have worked extensively with diverse state management systems in React,
						including Redux, MobX, and XState."
				.to_string(),
			img_uri: "images/logos/reactjs_logo.webp".to_string(),
			img_alt: "reactjs logo".to_string(),
		},
		ExpertiesRecord {
			title: "Node.js".to_string(),
			descrpition: "I have engaged in projects utilizing frameworks such as Express.js
						and Fastify, implementing various communication protocols like HTTP, WebSockets,
						and GraphQL. This hands-on experience has allowed me to build robust and
						efficient backend systems, ensuring seamless communication and optimal performance."
				.to_string(),
			img_uri: "images/logos/node_js_logo.webp".to_string(),
			img_alt: "node.js logo".to_string(),
		},
		ExpertiesRecord {
			title: "Docker".to_string(),
			descrpition: "Docker is crucial for diverse environment deployments and I have utilized
						it not only for deployment but also for testing with Testcontainers and streamlined
						development via Devcontainer. This comprehensive use of Docker ensures the
						consistancy through out the workflow."
				.to_string(),
			img_uri: "images/logos/docker_logo.webp".to_string(),
			img_alt: "docker logo".to_string(),
		},
		ExpertiesRecord {
			title: "Linux".to_string(),
			descrpition:
				"For more than a decade, Linux has been my primary OS. I have navigated significant
						tech shifts like X11 to Wayland, PulseAudio to PipeWire, and ext to btrfs file systems.
						With Linux now crucial in the SDLC, I'm grateful for my early fascination with it."
					.to_string(),
			img_uri: "images/logos/linux_logo.webp".to_string(),
			img_alt: "linux logo".to_string(),
		},
		ExpertiesRecord {
			title: "Git".to_string(),
			descrpition:
				"Being an open-source contributor, I deem Git, the version control tool, crucial
						for collaborative software development. Throughout my 7+ years in the field, Git has proven
						its value on numerous occasions. GitHub, along with GitHub Actions, enhances the open-source
						experience, streamlining CI/CD processes for a seamless workflow."
					.to_string(),
			img_uri: "images/logos/git_logo.webp".to_string(),
			img_alt: "git logo".to_string(),
		},
	];

	view! {
		<ExpertiesTemplate records={records} />
	}
}
