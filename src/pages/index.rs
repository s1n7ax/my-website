use yew::prelude::*;

use crate::components::Header;

#[function_component(Index)]
pub fn app() -> Html {
    html! {
        <div>
            <Header />
            <h1>{"Index"}</h1>
        </div>
    }
}
