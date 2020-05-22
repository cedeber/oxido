use seed::App;
use wasm_bindgen::prelude::*;

mod app;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    App::start(
        seed::browser::util::body(),
        app::init,
        app::update,
        app::view,
    );

    Ok(())
}
