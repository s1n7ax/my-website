use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    let src = format!("assets/images/me_01.webp");

    html! {
        <div class={classes!("flex", "justify-center", "lg:justify-center")}>
            <div class={classes!("w-2/3", "grid", "grid-cols-1", "grid-rows-1")}>
                <div class={classes!("col-start-1", "row-start-1")}>
                    <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
                        <path
                            class={classes!("fill-green-600")}
                            // stroke="#FF0066"
                            // fill="#FF0066"
                            // stroke-width="4"
                            // fill-opacity="1"
                            d="M39.9,-65.1C48.2,-56.8,48.8,-39.7,56.1,-25.2C63.3,-10.7,77.2,1.2,76.6,11.6C76,22,60.9,30.9,48.6,38.5C36.3,46.1,26.8,52.5,15.3,59C3.9,65.4,-9.6,71.8,-24.1,72.1C-38.6,72.3,-54.2,66.4,-66.4,55.7C-78.6,44.9,-87.4,29.4,-84.9,15.1C-82.4,0.9,-68.6,-12.1,-61.3,-28C-54.1,-44,-53.3,-62.9,-44.2,-70.9C-35.1,-78.8,-17.5,-75.7,-0.9,-74.4C15.8,-73,31.7,-73.5,39.9,-65.1Z"
                            transform="translate(100 100)"
                        />
                    </svg>
                </div>
                <div class={classes!("col-start-1", "row-start-1")}>
                    <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
                        <path class={classes!("fill-green-500")} d="M27.2,-45.5C35.3,-37.2,41.8,-29.6,50.1,-20.3C58.5,-10.9,68.7,0.2,73.1,14.6C77.4,29,75.9,46.7,65.7,55.1C55.5,63.4,36.8,62.4,20.7,64.3C4.6,66.2,-8.8,71,-24.4,71.6C-40,72.3,-57.7,68.9,-63,57.5C-68.3,46.2,-61.2,27,-58.1,11.8C-54.9,-3.4,-55.8,-14.6,-54.5,-28C-53.3,-41.4,-49.9,-57,-40.5,-64.6C-31.1,-72.3,-15.5,-72,-3,-67.4C9.6,-62.8,19.2,-53.8,27.2,-45.5Z" transform="translate(100 100)" />
                    </svg>
                </div>
                <div class={classes!("col-start-1", "row-start-1", "w-9/10")}>
                    <svg viewBox="50 50 100 100">
                        <defs>
                            <pattern
                                id="imgpattern"
                                width="1"
                                height="1"
                                viewBox="0 0 100 100"
                                preserveAspectRatio="xMidYMid slice"
                            >
                                <image width="100" height="100" href={ src }/>
                            </pattern>
                        </defs>
                        <path
                            fill="url(#imgpattern)"
                            d="M24.1,-28.6C31.3,-22.7,37.3,-15.1,37.5,-7.5C37.8,0.1,32.5,7.9,28,16.6C23.5,25.2,19.8,34.8,13.3,37.6C6.8,40.4,-2.5,36.5,-10.9,32.4C-19.4,28.3,-26.9,24.1,-30.4,17.7C-33.8,11.4,-33.2,2.8,-31.5,-5.3C-29.8,-13.5,-27,-21.2,-21.6,-27.5C-16.2,-33.7,-8.1,-38.5,0.2,-38.7C8.4,-38.9,16.9,-34.6,24.1,-28.6Z"
                            transform="translate(100 100)"
                        />
                    </svg>
                </div>
            </div>
        </div>
    }
}
