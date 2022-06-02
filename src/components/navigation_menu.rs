use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    html! {
        <nav class={classes!(
            "flex",
            "flex-row",
            "lg:flex-col",
            "justify-center",

            "w-full",
            "lg:w-24",
            "gap-2",
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
                "transition-all",

                "hover:w-max",
                "hover:rounded-lg",

                "bg-orange-600"
            )}
            to={props.route}
        >
            <img class={classes!(
                    "hidden",
                    "lg:inline-block",
                    "w-20",
                )}
                src={src}
                alt={props.alt.to_owned()}
            />
            <span class={classes!(
                "text-2xl",
                "mx-2",
                "block",
                "opacity-1",

                "transition-all",
                "duration-1000",

                "lg:opacity-0",
                "lg:hidden",
                "lg:group-hover:inline-block",
                "lg:group-hover:opacity-100",

            )}>
                { props.presentation.to_owned() }
            </span>
        </Link<Route>>
    }
}
