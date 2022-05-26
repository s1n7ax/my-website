use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct H1Props {
    pub children: Children
}

#[function_component(H1)]
pub fn h1(props: &H1Props) -> Html {
    html! {
        <h1 class={classes!(
            "text-5xl",
            "uppercase",
            "my-2"
        )}>
            { props.children.to_owned() }
        </h1>
    }
}
