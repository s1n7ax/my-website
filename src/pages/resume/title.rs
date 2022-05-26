use yew::prelude::*;

use crate::components::h1::H1;
use crate::components::h2::H2;

#[function_component(Title)]
pub fn title() -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-1")}>
            <H1>{ "Srinesh Nisala" }</H1>
            <H2>{ "Full Stack Engineer" }</H2>
        </div>
    }
}
