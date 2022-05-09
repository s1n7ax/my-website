use yew::prelude::*;

use crate::components::Body;

#[derive(Properties, PartialEq)]
struct CardProps {
    image: String,
    heading: String,
    subheading: String,
    content: String,
}

#[function_component(Card)]
fn card(props: &CardProps) -> Html {
    let src = format!("assets/images/{}.webp", props.image);

    html! {
        <div class={
            classes!(
                "flex",
                "flex-col",
                "lg:flex-row",
                "bg-yellow-400"
            )
        }>
            <div class={classes!(
                    "flex",
                    "justify-center",
                    "items-center",
                )
            }>
                <img
                    class={classes!(
                        "rounded-full",
                        "w-80",
                        "lg:w-96",
                        "h-auto"
                    )}
                    src={ src }
                />
            </div>

            <div class={classes!("p-10")}>
                <h1 class={classes!(
                        "uppercase",
                        "text-3xl"
                    )}
                >
                    { props.heading.to_owned() }
                </h1>

                <h1 class={classes!(
                        "uppercase",
                        "text-2xl"
                    )}
                >
                    { props.subheading.to_owned() }
                </h1>

                <p class={classes!(
                        "py-3"
                    )}
                >
                    { props.content.to_owned() }
                </p>
            </div>
        </div>
    }
}

#[function_component(Index)]
pub fn index() -> Html {
    let name = "Srinesh Nisala";
    let header = format!("I'm {}", name);

    html! {
        <Body>
            <div class={
                classes!(
                    "flex",
                    "justify-center",
                    "items-center",
                    "h-full",
                    "bg-red-400"
                )
            }>
                <Card
                    image="me_01"
                    heading={header}
                    subheading="Full Stack Engineer"
                    content="6+ years of hands-on experience designing, developing,
                        and implementing applications and solutions using a range of
                        technologies and programming languages. Seeking to leverage
                        broad development experience and hands-on technical expertise
                        in a challenging role as a Full-stack Developer."
                />
            </div>
        </Body>
    }
}
