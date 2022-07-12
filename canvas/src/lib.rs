use js_sys::{Int8Array, SharedArrayBuffer};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_wasm() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");
    //
    // let val = document.create_element("p")?;
    // val.set_text_content(Some("Hello from Rust!"));
    //
    // body.append_child(&val)?;

    // web_sys::console::log_1(&JsValue::from_str("Hello web-sys"));

    Ok(())
}

#[wasm_bindgen]
pub fn well(what: JsValue) -> Result<(), JsValue> {
    let data: SharedArrayBuffer = JsValue::into(what);
    let arr = Int8Array::new(&data);
    arr.set_index(0, 8);
    // web_sys::console::log_2(&JsValue::from_str("web-sys"), &data);
    Ok(())
}
