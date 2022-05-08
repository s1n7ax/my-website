use yew::prelude::*;

use crate::components::Body;

#[function_component(About)]
pub fn app() -> Html {
    html! {
        <Body>
            <h1>{"About Me"}</h1>
        </Body>
    }
}
