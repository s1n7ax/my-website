use yew::prelude::*;

#[function_component(Languages)]
pub fn languages() -> Html {
    html! {
        <div>
            <LanguageRecord
                lang="JavaScript"
                percentage={80}
            />
            <LanguageRecord
                lang="Java"
                percentage={60}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LanguageRecordProps {
    lang: String,
    percentage: u8,
}

#[function_component(LanguageRecord)]
fn language_record(props: &LanguageRecordProps) -> Html {
    html! {
        <div class={classes!("flex", "gap-2")}>
            <span> { props.lang.to_owned() } </span>
            <span> { format!("{}%", props.percentage) } </span>
        </div>
    }
}
