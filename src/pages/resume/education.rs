use yew::prelude::*;

use crate::pages::resume::H2Left as H2;

#[function_component(Education)]
pub fn education() -> Html {
    html! {
        <div>
            <H2>{ "Education" }</H2>
            <div class={classes!("flex", "flex-col", "gap-3")}>
                <EducationRecord
                    from="2020"
                    to="2021"
                    university="University of Bedfordshire"
                    localtion="England"
                    course="BSc (Hons) Computer Science & Software Engineering"
                    class="first class"
                    awards={vec![
                        "SLIIT Award — Best performance of the year".to_string(),
                        "SLIIT Award — Best research of the year".to_string()
                    ]}
                />
                <EducationRecord
                    from="2015"
                    to="2016"
                    university="National Institute of Business Management"
                    localtion="Sri Lanka"
                    course="Higher Diploma in Computer Based Information Systems"
                    class=""
                    awards={vec![]}
                />
                <EducationRecord
                    from="2014"
                    to="2015"
                    university="National Institute of Business Management"
                    localtion="Sri Lanka"
                    course="Diploma in Computer System Design"
                    class=""
                    awards={vec![]}
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct EducationRecordProps {
    from: String,
    to: String,
    university: String,
    localtion: String,
    course: String,
    class: String,
    awards: Vec<String>,
}

#[function_component(EducationRecord)]
fn education_record(props: &EducationRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "flex-col")}>
            <div class={classes!("font-bold")}>
                <span>{ format!("{} at {}, {}", props.course, props.university, props.localtion) }</span>
            </div>

            <div class={classes!("text-neutral-800")}>
                <span>{ format!("{} — {}", props.from, props.to) }</span>
            </div>

            <ul class={classes!("list-disc", "list-inside")}>
            {
                props
                    .awards
                    .to_owned()
                    .into_iter()
                    .map(|resp| html! {<li>{ resp }</li>})
                    .collect::<Vec<_>>()
            }
            </ul>
        </div>
    }
}
