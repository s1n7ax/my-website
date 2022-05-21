use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    let src = format!("assets/images/me_01.webp");

    html! {
        <div class={classes!()}>
            <svg class={classes!()} viewBox="15 15 150 150">
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
                    d="M36.7,-43.4C41.7,-31.6,36,-15.8,36.5,0.5C37,16.8,43.7,33.6,38.7,40.8C33.6,47.9,16.8,45.4,-1,46.4C-18.8,47.4,-37.5,51.8,-48.7,44.7C-59.8,37.5,-63.3,18.8,-61,2.3C-58.7,-14.2,-50.6,-28.4,-39.5,-40.2C-28.4,-52,-14.2,-61.4,0.8,-62.2C15.8,-63,31.6,-55.2,36.7,-43.4Z" transform="translate(100 100)"
                />
            </svg>
        </div>
    }
}
