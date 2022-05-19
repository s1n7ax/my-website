use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <div>
            <h2>{ "Profile" }</h2>
            <p>
                { "Motivated and resourceful engineer with 6 years’ of working
                experience in the industry. Primarily working with Java and
                web technologies. I seek to follow my passion in seeking new
                knowledge and creating it, and hope to do so under the
                guidance, presence of great scientific minds, and contribute
                my maximum towards the benefit of academia as a whole, and
                mankind in turn." }
            </p>
        </div>
    }
}
