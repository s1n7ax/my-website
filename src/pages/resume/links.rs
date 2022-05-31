use yew::prelude::*;

use crate::components::H2;

#[function_component(Links)]
pub fn links() -> Html {
    html! {
        <div>
            <H2>{ "Links" }</H2>
            <div class={classes!("flex", "flex-col", "gap-3")}>
                <Icon
                    icon_name="github-brands"
                    presentation="GitHub"
                    href="https://github.com/s1n7ax"
                />
                <Icon
                    icon_name="youtube-brands"
                    presentation="YouTube"
                    href="https://www.youtube.com/c/s1n7ax"
                />
                <Icon
                    icon_name="twitter-brands"
                    presentation="Twitter"
                    href="https://twitter.com/s1n7ax"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct IconProps {
    presentation: String,
    icon_name: String,
    href: String,
}

#[function_component(Icon)]
fn icon(props: &IconProps) -> Html {
    let src = format!("assets/icons/{}.svg", props.icon_name.to_owned());

    html! {
        <a
            class={classes!("flex", "items-center", "gap-2")}
            href={props.href.to_owned()}
            target="_blank"
        >
            <img class={classes!("w-10")} src={src} />
            <span>{ props.presentation.to_owned() }</span>
        </a>
    }
}
