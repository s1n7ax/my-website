use yew::prelude::*;

use crate::components::H2;

#[function_component(Technologies)]
pub fn technologies() -> Html {
    html! {
        <div>
            <H2>{ "Technologies" }</H2>
            <LanguageRecord
                lang="JavaScript"
                percentage={80}
            />
            <LanguageRecord
                lang="Lua"
                percentage={80}
            />
            <LanguageRecord
                lang="Python"
                percentage={60}
            />
            <LanguageRecord
                lang="Java"
                percentage={60}
            />
            <LanguageRecord
                lang="Rust"
                percentage={20}
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
