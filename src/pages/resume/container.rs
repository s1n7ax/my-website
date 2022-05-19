use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    html! {
        <div class={
            classes!(
                "flex",
                "my-10",
                "p-10",
                "bg-yellow-400",
            )
        }>
            {props.children.to_owned()}
        </div>
    }
}
