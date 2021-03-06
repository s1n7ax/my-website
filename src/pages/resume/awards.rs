use yew::prelude::*;

use crate::components::H2;

#[function_component(Awards)]
pub fn awards() -> Html {
    html! {
        <div>
            <H2>{ "Awards" }</H2>
            <AwardsRecord
                year="2021"
                description="SLIIT Best Performance of the Year"
            />
            <AwardsRecord
                year="2021"
                description="SLIIT Best Research of the Year"
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AwardsRecordProps {
    year: String,
    description: String,
}

#[function_component(AwardsRecord)]
fn awards_record(props: &AwardsRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "gap-2")}>
            <span>{ props.year.to_owned() }</span>
            <span>{ props.description.to_owned() }</span>
        </div>
    }
}
