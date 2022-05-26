use yew::prelude::*;

use crate::components::H2;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <div>
            <H2>{ "Employment History" }</H2>
            <div class={classes!("flex", "flex-col", "gap-3")}>
                <WorkRecord
                    from="MAR 2021"
                    to="Present"
                    company="Mediconsult Oy"
                    designation="Full Stack Engineer"
                    responsibilities={vec![
                        "Building/maintaining CICD infrastructure".to_string(),
                        "Developing micro-services to migrate old monolith applications in Finland health care sector".to_string(),
                    ]}
                />
                <WorkRecord
                    from="SEP 2017"
                    to="MAR 2021"
                    company="Infor"
                    designation="QA Automation Engineer"
                    responsibilities={vec![
                        "Led in-house REST spec/test platform used by about 100 companies around the world".to_string(),
                        "Collaborated with many teams to onboard users to UI action recorder-playback tool".to_string(),

                    ]}
                />
                <WorkRecord
                    from="FEB 2016"
                    to="SEP 2017"
                    company="Virtusa"
                    designation="Associate Automation Specialist"
                    responsibilities={vec![
                        "Contributed to in-house UI automation tool used by more than 300 QA Engineers".to_string(),
                        "Integrated HTTP client features to automation tools".to_string(),
                    ]}
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
    responsibilities: Vec<String>,
}

#[function_component(WorkRecord)]
fn work_record(props: &WorkRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "flex-col")}>
            <div class={classes!("font-bold")}>
                <span>{ format!("{} at {}", props.designation, props.company) }</span>
            </div>

            <div class={classes!("text-neutral-800")}>
                <span>{ format!("{} — {}", props.from, props.to) }</span>
            </div>

            <ul class={classes!("list-disc", "list-inside")}>
            {
                props
                    .responsibilities
                    .to_owned()
                    .into_iter()
                    .map(|resp| html! {<li>{ resp }</li>})
                    .collect::<Vec<_>>()
            }
            </ul>
        </div>
    }
}
