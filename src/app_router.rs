use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{About, CoverLetter, Index, NotFound};

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("about")]
    About,

    #[at("/cover_letter")]
    CoverLetter,

    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::About => html! { <About /> },
        Route::CoverLetter => html! { <CoverLetter /> },
        _ => html! { <NotFound /> },
    }
}
