use yew::prelude::*;

use crate::components::H2;

#[function_component(Technologies)]
pub fn technologies() -> Html {
    html! {
        <div>
            <H2>{ "Technologies" }</H2>
            <div class={classes!("flex", "flex-col", "gap-2")}>
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
        <div>
            <div class={classes!(
                "mb-1",
                "text-base",
                "font-medium",
            )}>
                { props.name.to_owned() }
            </div>

            <div class={classes!(
                "w-full",
                "bg-gray-200",
                "rounded-full",
                "dark:bg-gray-700"
            )}>
                <div
                    class={classes!(
                        "text-xs",
                        "font-medium",
                        "text-blue-100",
                        "text-center",
                        "p-0.5",
                        "leading-none",
                        "rounded-full",
                        "bg-orange-500",
                    )}
                    style={ format!("width: {}%", props.percentage) }
                >
                    { format!("{}%", props.percentage) }
                </div>
            </div>
        </div>
    }
}
