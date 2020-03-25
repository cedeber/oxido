use core::cell::Cell;
use std::rc::Rc;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, MouseEvent};

pub fn draw(
    context: &CanvasRenderingContext2d,
    event: &Option<MouseEvent>,
    width: &u32,
    height: &u32,
    pressed_position: &Rc<Cell<Option<(i32, i32)>>>,
) {
    context.clear_rect(0.0, 0.0, *width as f64, *height as f64);

    // Fill the background
    context.begin_path();
    context.set_fill_style(&JsValue::from("#100707"));
    context.fill_rect(0.0, 0.0, *width as f64, *height as f64);

    context.begin_path();
    context.set_line_width(1.0);
    context.set_stroke_style(&JsValue::from("#D392E6"));

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

    context.stroke();

    // Draw the mouse lines
    match event {
        Some(e) => {
            // Draw the select rectangle
            match pressed_position.get() {
                Some(p) => {
                    let width = e.offset_x() - p.0;
                    let height = e.offset_y() - p.1;

                    context.begin_path();
                    context.set_stroke_style(&JsValue::from("dodgerblue"));
                    context.rect(p.0 as f64, p.1 as f64, width.into(), height.into());
                    context.stroke();
                }
                None => {}
            }

            context.begin_path();
            context.set_stroke_style(&JsValue::from("#A61B85"));
            context.move_to(e.offset_x() as f64, 0.0);
            context.line_to(e.offset_x() as f64, *height as f64);
            context.move_to(0.0, e.offset_y() as f64);
            context.line_to(*width as f64, e.offset_y() as f64);
            context.stroke();
        }
        None => {}
    }
}
