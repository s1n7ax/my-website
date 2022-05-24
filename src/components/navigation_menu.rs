use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub route: Route,
    pub alt: String,
    pub icon_name: String,
    pub presentation: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let src = format!("assets/icons/{}.svg", props.icon_name);

    html! {
        <Link<Route> classes={
            classes!(
                "group",
                "flex",
                "items-center",
                "p-2",
                "z-10",

                "hover:w-max",
                "hover:rounded-lg",
                "transition-all",

                "bg-orange-600"
            )}
            to={props.route}
        >
            <img class={classes!(
                    "w-20",
                )}
                src={src}
                alt={props.alt.to_owned()}
            />
            <span class={classes!(
                "text-2xl",
                "mx-2",
                "hidden",
                "opacity-0",

                "group-hover:inline-block",
                "group-hover:opacity-100",
                "transition-all",
                "duration-1000"
            )}>
                { props.presentation.to_owned() }
            </span>
        </Link<Route>>
    }
}

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    html! {
        <nav class={classes!(
            "flex",
            "flex-col",
            "justify-center",

            "m-2",
            "w-24",
            "gap-2",

            "bg-blue-600",
        )}>
            <Icon
                presentation="Home"
                route={Route::Index} icon_name="house-chimney-solid"
                alt="Home"
            />
            <Icon
                presentation="Resume"
                route={Route::Resume} icon_name="address-card-solid"
                alt="Resume"
            />
            <Icon
                presentation="Portfolio"
                route={Route::Portfolio} icon_name="user-graduate-solid"
                alt="Portfolio"
            />
            <Icon
                presentation="Contact"
                route={Route::Contact} icon_name="address-book-solid"
                alt="Contact"
            />
            <Icon
                presentation="About"
                route={Route::About} icon_name="circle-info-solid"
                alt="About"
            />
        </nav>
    }
}
