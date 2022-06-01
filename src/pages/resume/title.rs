use yew::prelude::*;

use crate::components::h1::H1;
use crate::components::h2::H2;

#[function_component(Title)]
pub fn title() -> Html {
    html! {
        <div class={classes!(
            "flex",
            "h-full",
            "items-center",
        )}>
            <div class={classes!(
                "flex",
                "flex-col",
            )}>
                <H1>{ "Srinesh Nisala" }</H1>
                <H2>{ "Full Stack Engineer" }</H2>
            </div>
        </div>
    }
}
