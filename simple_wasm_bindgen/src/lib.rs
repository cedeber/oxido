use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // From Javascript
    fn wasm_cb(s: &str);

    // The `async` can be combined with the `catch` attribute to manage errors from the JS promise
    #[wasm_bindgen(catch)]
    async fn async_wasm_cb(s: &str) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    wasm_cb("Hello from Rust!");
    a + b
}

#[wasm_bindgen]
pub async fn async_add(a: i32, b: i32) -> Result<i32, JsValue> {
    log("[ASYNC] async_add() called");
    // Wait from JS
    let c = async_wasm_cb("Hello from Async Rust!").await?.as_f64().unwrap() as i32;
    log("[ASYNC] will return addition result");
    Ok(a + b + c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, add(2, 3));
    }
}
