use yew::prelude::*;

use crate::components::body::Body;
use crate::pages::resume::{
    Contact, Container, Education, Experience, Image, Links, Profile, Technologies, Title,
};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <Body>
            <Container>
                <div class={classes!(
                    "grid",
                    "grid-cols-1",
                    "lg:grid-cols-10",
                    "gap-3",

                    "w-9/10",
                    "xl:max-w-[80rem]",
                )}>
                    <div class={classes!(
                        "lg:col-span-5",

                        "bg-red-400"
                    )}>
                        <Title />
                    </div>

                    <div class={classes!(
                        "lg:col-span-5",
                        "bg-red-400",
                        "lg:order-none",

                        "order-first",
                    )}>
                        <Image />
                    </div>

                    <div class={classes!(
                        "lg:col-span-10",

                        "bg-red-400"
                    )}>
                        <Contact />
                    </div>

                    <div class={classes!(
                        "lg:col-span-10",

                        "bg-red-400"
                    )}>
                        <Profile />
                    </div>

                    <div class={classes!(
                        "lg:col-span-7",
                        "lg:row-span-2",
                        "lg:col-start-4",

                        "bg-red-400"
                    )}>
                        <Experience />
                    </div>

                    <div class={classes!(
                        "lg:col-span-7",
                        "lg:col-start-4",

                        "bg-red-400"
                    )}>
                        <Education />
                    </div>

                    <div class={classes!(
                        "lg:col-span-3",
                        "lg:row-start-4",

                        "bg-red-400"
                    )}>
                        <Links />
                    </div>

                    <div class={classes!(
                        "lg:col-span-3",
                        "lg:row-span-2",
                        "lg:row-start-5",

                        "bg-red-400"
                    )}>
                        <Technologies />
                    </div>
                </div>
            </Container>
        </Body>
    }
}
