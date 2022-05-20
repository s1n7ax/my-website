use yew::prelude::*;

use crate::components::H2;

#[function_component(Technologies)]
pub fn technologies() -> Html {
    html! {
        <div>
            <H2>{ "Technologies" }</H2>
            <TechnologyRecord
                name="ReactJs"
                percentage={80}
            />
            <TechnologyRecord
                name="Angular"
                percentage={30}
            />
            <TechnologyRecord
                name="Spring Boot"
                percentage={40}
            />
            <TechnologyRecord
                name="Typescript"
                percentage={80}
            />
            <TechnologyRecord
                name="Python"
                percentage={60}
            />
            <TechnologyRecord
                name="Java"
                percentage={60}
            />
            <TechnologyRecord
                name="Rust"
                percentage={20}
            />
            <TechnologyRecord
                name="Selenium"
                percentage={80}
            />
            <TechnologyRecord
                name="Cypress"
                percentage={80}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TechnologyRecordProps {
    name: String,
    percentage: u8,
}

#[function_component(TechnologyRecord)]
fn technology_record(props: &TechnologyRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "gap-2")}>
            <span> { props.name.to_owned() } </span>
            <span> { format!("{}%", props.percentage) } </span>
        </div>
    }
}
