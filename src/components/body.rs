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
        <div class={classes!(
            "flex",
            "flex-col",
            "lg:flex-row",
            "h-screen",
        )}>
            <div class={classes!(
                "flex",
            )}>
                <NavigationMenu />
            </div>

            <div class={classes!(
                "flex-grow",
                "overflow-auto",
            )}>
                {props.children.to_owned()}
            </div>
        </div>
    }
}
