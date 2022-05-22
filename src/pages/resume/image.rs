use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    let src = format!("assets/images/me_01.webp");

    html! {
        <div class={classes!("flex", "justify-center", "lg:justify-end")}>
            <div class={classes!("w-1/2")}>
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
    }
}
