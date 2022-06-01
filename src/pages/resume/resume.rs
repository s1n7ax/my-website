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
                    "rounded",
                    "bg-neutral-200",
                    "p-10",
                    "hover:drop-shadow-lg",
                    "transition-all",
                    "duration-200",
                )}>
                    <div class={classes!(
                        "grid",
                        "grid-cols-1",
                        "gap-10",

                        "lg:grid-cols-10",
                        "w-9/10",
                        "xl:max-w-[80rem]",
                    )}>
                        <div class={classes!(
                            "lg:col-span-5",
                        )}>
                            <Title />
                        </div>

                        <div class={classes!(
                            "lg:col-span-5",
                            "lg:order-none",

                            "order-first",
                        )}>
                            <Image />
                        </div>

                        <div class={classes!(
                            "lg:col-span-10",
                        )}>
                            <Contact />
                        </div>

                        <div class={classes!(
                            "lg:col-start-4",
                            "lg:col-span-7",
                        )}>
                            <Profile />
                        </div>

                        <div class={classes!(
                            "lg:col-span-7",
                            "lg:row-span-3",
                            "lg:col-start-4",
                        )}>
                            <Experience />
                        </div>

                        <div class={classes!(
                            "lg:col-span-7",
                            "lg:col-start-4",
                        )}>
                            <Education />
                        </div>

                        <div class={classes!(
                            "lg:col-span-3",
                            "lg:row-span-2",

                            "lg:row-start-3",
                        )}>
                            <Links />
                        </div>

                        <div class={classes!(
                            "lg:col-span-3",
                            "lg:row-span-3",
                            "lg:row-start-5",
                        )}>
                            <Technologies />
                        </div>
                    </div>
                </div>
            </Container>
        </Body>
    }
}
