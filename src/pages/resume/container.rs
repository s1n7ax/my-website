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
                "flex-col",
                "justify-center",
                "items-center",
                "overflow-auto",

                "min-h-full",

                "px-10",
                "py-10",
            )
        }>
            {props.children.to_owned()}
        </div>
    }
}
