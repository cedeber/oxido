mod components;

use crate::components::adder::Adder;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div class={css!("color: crimson;")}>
            <Adder qtty={2} />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
