use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    let src = format!("assets/images/me_01.webp");

    html! {
        <img class={classes!("w-96")} src={src} />
    }
}
