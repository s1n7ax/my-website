use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div>
            <div class={classes!(
                    "lg:flex",
                    "lg:justify-center",
                    "lg:gap-5"
            )}>
                <ContactInfo
                    icon_name="phone-solid"
                    presentation="+94777398803"
                    href="tel: +94777398803"
                />
                <ContactInfo
                    icon_name="envelope-open-text-solid"
                    presentation="srineshnisala@gmail.com"
                    href="mailto: srineshnisala@gmail.com"
                />
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
    let icon = format!("assets/icons/{}.svg", props.icon_name.to_owned());

    html! {
        <div class={classes!("flex", "gap-1")}>
            <img class={classes!("w-5")} src={icon.to_owned()} />
            <a class={classes!(
                    "float-left",
                    "text-2xl"
                )}
                href={props.href.to_owned()}
                target="_blank">{ props.presentation.to_owned() }
            </a>
        </div>
    }
}
