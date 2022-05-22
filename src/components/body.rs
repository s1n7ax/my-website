use crate::components::NavigationMenu;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Body)]
pub fn body(props: &Props) -> Html {
    html! {
        <div class={classes!("flex", "h-screen")}>
            // navigation menu
            <div
                class={classes!(
                        "flex",
                        "flex-col",
                        "justify-center",
                        "bg-green-300")}>
                <NavigationMenu />
            </div>

            // content
            <div class={
                classes!(
                    "flex-grow",
                    "overflow-auto",
                    "bg-red-200",
                )}
            >
                {props.children.to_owned()}
            </div>
        </div>
    }
}
