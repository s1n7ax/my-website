use yew::prelude::*;

use crate::components::Body;
use crate::pages::home::container::Container;
use crate::pages::home::info_card::{InfoCard, Justify};

#[function_component(Home)]
pub fn home() -> Html {
    let name = "Srinesh Nisala";
    let header = format!("I'm {}", name);

    html! {
        <Body>
            <div class={
                classes!(
                    "flex",
                    "flex-col",
                    "justify-center",
                    "items-center",
                    "h-full",
                    "bg-red-400"
                )
            }>
                <Container>
                    <InfoCard
                        justify={Justify::Start}
                        image="me_01"
                        heading={header.to_owned()}
                        subheading="Full Stack Engineer"
                    >
                        {
                            "6+ years of hands-on experience designing, developing,
                            and implementing applications and solutions using a range of
                            technologies and programming languages. Seeking to leverage
                            broad development experience and hands-on technical expertise
                            in a challenging role as a Full-stack Developer."
                        }
                    </InfoCard>
                </Container>
            </div>
        </Body>
    }
}
