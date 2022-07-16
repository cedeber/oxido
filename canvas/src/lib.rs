use js_sys::{SharedArrayBuffer, Uint8ClampedArray};
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
pub fn well(what: JsValue, width: u32, height: u32) -> Result<(), JsValue> {
    let data: SharedArrayBuffer = JsValue::into(what);
    let arr = Uint8ClampedArray::new(&data);

    for y in 0..height {
        let in_top = (y / (height / 2)) == 0;
        for x in 0..width {
            let in_left = (x / (width / 2)) == 0;
            let ix = (y * width + x) * 4;
            let (r, g, b) = match (in_top, in_left) {
                (true, true) => (0xff, 0x00, 0x00),
                (true, false) => (0x00, 0xff, 0x00),
                (false, true) => (0x00, 0x00, 0xff),
                (false, false) => (0x00, 0x00, 0x00),
            };
            arr.set_index(ix, r);
            arr.set_index(ix + 1, g);
            arr.set_index(ix + 2, b);
            arr.set_index(ix + 3, 0x77);
        }
    }

    // web_sys::console::log_2(&JsValue::from_str("web-sys"), &data);
    Ok(())
}
