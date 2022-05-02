#![recursion_limit = "640"]

mod app_router;
mod components;
mod pages;

use app_router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
