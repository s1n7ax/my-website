use crate::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Copy, Debug)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("about")]
    About,

    #[at("/contact")]
    Contact,

    #[at("/portfolio")]
    Portfolio,

    #[at("*")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::Portfolio => html! { <Portfolio /> },
        _ => html! { <NotFound /> },
    }
}
