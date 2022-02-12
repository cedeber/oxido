mod components;

use crate::components::adder::Adder;
use stylist::yew::styled_component;
use yew::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    #[cfg(debug_assertions)]
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    yew::start_app::<App>();
}
