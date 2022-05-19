use yew::prelude::*;

#[function_component(Title)]
pub fn title() -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-1")}>
            <h1 class={classes!("text-5xl")}>{ "Srinesh Nisala" }</h1>
            <h2 class={classes!("text-3xl")}>{ "Full Stack Engineer" }</h2>
        </div>
    }
}
