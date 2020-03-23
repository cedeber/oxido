use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

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
pub fn setup(width: u32, height: u32) -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(width);
    canvas.set_height(height);
    canvas.style().set_property("border", "solid")?;
    canvas
        .style()
        .set_property("image-rendering", "crisp-edges")?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let context = Rc::new(context);

    {
        // MouseEvent Closure
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            draw(&context, Some(event), &width, &height)
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    draw(&context, None, &width, &height);

    Ok(())
}

fn draw(context: &CanvasRenderingContext2d, event: Option<MouseEvent>, width: &u32, height: &u32) {
    context.clear_rect(0.0, 0.0, *width as f64, *height as f64);
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouse lines
    match event {
        Some(e) => {
            context.move_to(e.offset_x() as f64, 0.0);
            context.line_to(e.offset_x() as f64, *height as f64);
            context.move_to(0.0, e.offset_y() as f64);
            context.line_to(*width as f64, e.offset_y() as f64);
        }
        None => {}
    }

    context.stroke();
}
