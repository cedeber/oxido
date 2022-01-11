mod components;

use crate::components::adder::Adder;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        // language=SCSS prefix={ suffix=}
        <div class={css!("color: #242526;")}>
            <Adder qtty={2} />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
