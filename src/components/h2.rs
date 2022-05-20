use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct H2Props {
    pub children: Children
}

#[function_component(H2)]
pub fn h2(props: &H2Props) -> Html {
    html! {
        <h2 class={classes!("text-2xl", "my-2")}>{ props.children.to_owned() }</h2>
    }
}
