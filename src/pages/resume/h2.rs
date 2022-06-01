use yew::prelude::*;

use crate::components::H2 as H2Org;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(H2Left)]
pub fn h2_left(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "flex",
            "basis-full",
            "items-center",

            "before:grow-[1]",
            "before:h-[1px]",
            "before:bg-black",
            "before:mr-2",

            "after:grow-[5]",
            "after:h-[1px]",
            "after:bg-black",
            "after:ml-2",
        )}>
            <H2Org>{ props.children.to_owned() }</H2Org>
        </div>
    }
}


#[function_component(H2Center)]
pub fn h2_center(props: &Props) -> Html {
    html! {
        <div class={classes!(
            "flex",
            "basis-full",
            "items-center",

            "before:h-[1px]",
            "before:bg-black",
            "before:mr-2",
            "before:grow-[1]",
            "lg:before:grow",

            "after:h-[1px]",
            "after:bg-black",
            "after:ml-2",
            "after:grow-[5]",
            "lg:after:grow"
        )}>
            <H2Org>{ props.children.to_owned() }</H2Org>
        </div>
    }
}
