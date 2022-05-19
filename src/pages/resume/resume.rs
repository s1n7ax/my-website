use yew::prelude::*;

use crate::components::body::Body;
use crate::pages::resume::{
    Awards, Contact, Container, Education, Experience, Image, Languages, Profile, Title,
};

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <Body>
            <Container>
                <div class={classes!("grid", "grid-cols-3", "gap-3", "w-full")}>
                    <div class={classes!("bg-red-400")}><Title /></div>
                    <div class={classes!("bg-red-400")}><Image /></div>
                    <div class={classes!("bg-red-400")}><Contact /></div>
                    <div class={classes!("bg-red-400")}><Profile /></div>
                    <div class={classes!("bg-red-400")}><Experience /></div>
                    <div class={classes!("bg-red-400")}><Education /></div>
                    <div class={classes!("bg-red-400")}><Awards /></div>
                    <div class={classes!("bg-red-400")}><Languages /></div>
                </div>
            </Container>
        </Body>
    }
}
