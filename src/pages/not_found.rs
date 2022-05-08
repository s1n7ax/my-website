use yew::prelude::*;

use crate::components::Body;

#[function_component(NotFound)]
pub fn app() -> Html {
    html! {
        <Body>
            <h1>{"Not Found"}</h1>
        </Body>
    }
}
