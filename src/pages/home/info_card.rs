use yew::prelude::*;

#[derive(PartialEq)]
pub enum Justify {
    Start,
    End,
    Center,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub justify: Justify,
    pub image: String,
    pub heading: String,
    pub subheading: String,
    pub children: Children,
}

#[function_component(InfoCard)]
pub fn info_card(props: &Props) -> Html {
    let src = format!("assets/images/{}.webp", props.image);

    let flex_classes = match props.justify {
        Justify::Start => classes!("lg:flex-row", "justify-start"),
        Justify::End => classes!("lg:flex-row-reverse", "justify-start"),
        Justify::Center => classes!("justify-center"),
    };

    html! {
        <div class={classes!(
                { flex_classes },
                "flex",
                "flex-col",
            )
        }>
            // card image
            <div class={classes!(
                    "flex",
                    "justify-center",
                    "items-center",
                )
            }>
                <img
                    class={classes!(
                        "w-80",
                        "rounded-full",
                        "lg:rounded-3xl",
                        "lg:w-96",
                        "h-auto"
                    )}
                    src={ src }
                />
            </div>

            <div class={classes!(
                "flex",
                "flex-col",
                "justify-center",
                "p-10",
                "lg:max-w-[60%]"
            )}>
                // card heading
                <h1 class={classes!(
                        "uppercase",
                        "text-3xl"
                    )}
                >
                    { props.heading.to_owned() }
                </h1>

                // card subheading
                <h1 class={classes!(
                        "uppercase",
                        "text-2xl"
                    )}
                >
                    { props.subheading.to_owned() }
                </h1>

                // card content
                <p class={classes!(
                        "py-3"
                    )}
                >
                    { props.children.to_owned() }
                </p>
            </div>
        </div>
    }
}
