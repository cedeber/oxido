mod components;

use crate::components::adder::Model;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Model qtty={2} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
