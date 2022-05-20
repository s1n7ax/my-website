use yew::prelude::*;

use crate::components::H2;

#[function_component(Education)]
pub fn education() -> Html {
    html! {
        <div>
            <H2>{ "Education" }</H2>
            <EducationRecord
                from="2014"
                to="2015"
                university="NIBM"
                course="Diploma in Computer System Design"
                awards=""
            />
            <EducationRecord
                from="2015"
                to="2016"
                university="NIBM"
                course="Higher Diploma in Computer Based Information Systems"
                awards=""
            />
            <EducationRecord
                from="2020"
                to="2021"
                university="University of Bedfordshire, England"
                course="BSc (Hons) Computer Science & Software Engineering"
                awards="(first class)"
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct EducationRecordProps {
    from: String,
    to: String,
    university: String,
    course: String,
    awards: String,
}

#[function_component(EducationRecord)]
fn education_record(props: &EducationRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "gap-2")}>
            <span>{ props.from.to_owned() }</span>
            <span>{ "-" }</span>
            <span>{ props.to.to_owned() }</span>
            <span>
                {
                    props.university.to_owned()
                    + props.course.as_str()
                    + props.awards.as_str()
                }
            </span>
        </div>
    }
}
