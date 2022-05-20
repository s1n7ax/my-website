use yew::prelude::*;

use crate::components::H2;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <div>
            <H2>{ "Experience" }</H2>
            <div class={classes!("flex", "flex-col")}>
                <WorkRecord
                    from="FEB 2016"
                    to="SEP 2017"
                    company="Virtusa"
                    designation="Associate Automation Specialist"
                />
                <WorkRecord
                    from="SEP 2017"
                    to="MAR 2021"
                    company="Infor"
                    designation="QA Automation Engineer"
                />
                <WorkRecord
                    from="MAR 2021"
                    to="Current"
                    company="Mediconsult Oy"
                    designation="Full Stack Engineer"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct WorkRecordProps {
    from: String,
    to: String,
    company: String,
    designation: String,
}

#[function_component(WorkRecord)]
fn work_record(props: &WorkRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "flex-row", "gap-1")}>
            <span>{ props.from.to_owned() }</span>
            <span>{ "-" }</span>
            <span>{ props.to.to_owned() }</span>
            <span class="font-bold"> { props.company.to_owned() } </span>
            <span> { "—	" } </span>
            <span> { props.designation.to_owned() } </span>
        </div>
    }
}
