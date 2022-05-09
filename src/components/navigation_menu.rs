use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub route: Route,
    pub alt: String,
    pub icon_name: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let src = format!("assets/icons/{}.svg", props.icon_name);

    html! {
        <Link<Route> classes={classes!("m-2",
                                       "p-2",
                                       "w-20",
                                       "h-20",
                                       "bg-orange-200")}
        to={props.route}>
            <img
                src={src}
                alt={props.alt.to_owned()}
            />
        </Link<Route>>
    }
}

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    html! {
        // Navbar
        <nav class={classes!("flex", "flex-col", "py-10")}>
            <Icon route={Route::Index} icon_name="house-chimney-solid" alt="Home" />
            <Icon route={Route::Portfolio} icon_name="user-graduate-solid" alt="Portfolio" />
            <Icon route={Route::Contact} icon_name="address-card-solid" alt="Contact" />
            <Icon route={Route::About} icon_name="circle-info-solid" alt="About" />
        </nav>
    }
}
