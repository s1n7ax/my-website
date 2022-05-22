use yew::prelude::*;

use crate::components::body::Body;
use crate::pages::resume::{
    Awards, Contact, Container, Education, Experience, Image, Technologies, Profile, Title,
};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <Body>
            <Container>
                <div
                    class={
                        classes!(
                            "grid",
                            "grid-cols-1",
                            "lg:grid-cols-10",
                            "gap-3",

                            "w-9/10",
                            "xl:max-w-[80rem]",
                        )
                }>
                    <div class={classes!(
                        "lg:col-span-5",
                        "lg:row-span-2",
                        "bg-red-400"
                    )}>
                        <Title />
                    </div>

                    <div class={classes!(
                        "lg:col-span-5",
                        "lg:row-span-3",
                        "bg-red-400",
                        "order-first",
                        "lg:order-none"
                    )}>
                        <Image />
                    </div>

                    <div class={classes!(
                        "lg:col-span-5",
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
                        "lg:col-span-5",
                        "bg-red-400"
                    )}>
                        <Experience />
                    </div>

                    <div class={classes!(
                        "lg:col-span-5",
                        "bg-red-400"
                    )}>
                        <Education />
                    </div>

                    <div class={classes!(
                        "lg:col-span-6",
                        "bg-red-400"
                    )}>
                        <Technologies />
                    </div>

                    <div class={classes!(
                        "lg:col-span-4",
                        "bg-red-400"
                    )}>
                        <Awards />
                    </div>
                </div>
            </Container>
        </Body>
    }
}

