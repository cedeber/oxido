use crate::utils::draw;
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn setup(id: String, width: u32, height: u32) -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document.get_element_by_id(&id).unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(width);
    canvas.set_height(height);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);
    let pressed_position = Rc::new(Cell::new(None));

    {
        // MouseDown Event Closure
        let pressed_position = pressed_position.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            pressed_position.set(Some((event.offset_x(), event.offset_y())));
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        // MouseUp Event Closure
        let context = context.clone();
        let pressed_position = pressed_position.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            pressed_position.set(None);
            draw(&context, &Some(event), &width, &height, &pressed_position);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        // MouseMove Event Closure
        let context = context.clone();
        let pressed_position = pressed_position.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            draw(&context, &Some(event), &width, &height, &pressed_position);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        // MouseOut Event Closure
        let context = context.clone();
        let pressed_position = pressed_position.clone();
        let closure = Closure::wrap(Box::new(move |_: MouseEvent| {
            pressed_position.set(None);
            draw(&context, &None, &width, &height, &pressed_position);
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    draw(&context, &None, &width, &height, &pressed_position);

    Ok(())
}
