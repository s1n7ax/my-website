use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <div class={classes!("grid", "grid-cols-1")}>
                <ContactInfo
                    icon_name="phone-solid"
                    presentation="+94 77 739 8803"
                    href="tel: +94777398803"
                />
                <ContactInfo
                    icon_name="envelope-open-text-solid"
                    presentation="srineshnisala@gmail.com"
                    href="mailto: srineshnisala@gmail.com"
                />
                <ContactInfo
                    icon_name="linkedin-brands"
                    presentation="linkedin.com/in/srinesh-nisala"
                    href="https://www.linkedin.com/in/srinesh-nisala/"
                />
            </div>
            <div class={classes!("flex", "flex-row", "my-5", "gap-4")}>
                <Icon icon_name="github-brands" href="https://github.com/s1n7ax" />
                <Icon icon_name="youtube-brands" href="https://www.youtube.com/c/s1n7ax" />
                <Icon icon_name="twitter-brands" href="https://twitter.com/s1n7ax" />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ContactInfoProps {
    icon_name: String,
    href: String,
    presentation: String,
}

#[function_component(ContactInfo)]
fn contact_info(props: &ContactInfoProps) -> Html {
    let icon = get_svg_src(props.icon_name.to_owned());

    html! {
        <div class={classes!("flex", "flex-row", "gap-5")}>
            <img class={classes!("w-5")} src={icon.to_owned()} />
            <a
                class={classes!("float-left", "text-2xl")}
                href={props.href.to_owned()}
                target="_blank">{ props.presentation.to_owned() }
            </a>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct IconProps {
    icon_name: String,
    href: String,
}

#[function_component(Icon)]
fn icon(props: &IconProps) -> Html {
    let src = get_svg_src(props.icon_name.to_owned());

    html! {
        <a href={props.href.to_owned()} target="_blank">
            <img class={classes!("w-10")} src={src} />
        </a>
    }
}

fn get_svg_src(name: String) -> String {
    format!("assets/icons/{}.svg", name)
}
