use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

// `init` describes what should happen when your app started.
pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Rendered);
    Model::default()
}

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    amount: i32,
    event_streams: Vec<StreamHandle>,
    point: Point,
    key_code: u32,
    fill_color: Color,
    canvas: ElRef<HtmlCanvasElement>,
}

#[derive(Default)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    A,
    B,
}

impl Color {
    fn as_str(&self) -> &str {
        match self {
            Self::A => "white",
            Self::B => "green",
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::A
    }
}

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    Increment,
    ToggleWatching,
    MouseMoved(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent),
    Rendered,
    ChangeColor,
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.amount += 1,
        Msg::ToggleWatching => {
            if model.event_streams.is_empty() {
                model.event_streams = vec![
                    orders.stream_with_handle(streams::window_event(Ev::MouseMove, |event| {
                        Msg::MouseMoved(event.unchecked_into())
                    })),
                    orders.stream_with_handle(streams::window_event(Ev::KeyDown, |event| {
                        Msg::KeyPressed(event.unchecked_into())
                    })),
                ];
            } else {
                model.event_streams.clear();
            }
        }
        Msg::MouseMoved(ev) => {
            model.point = Point {
                x: ev.client_x(),
                y: ev.client_y(),
            }
        }
        Msg::KeyPressed(ev) => model.key_code = ev.key_code(),
        Msg::Rendered => {
            draw(&model.canvas, model.fill_color);
            // We want to call `.skip` to prevent infinite loop.
            // (However infinite loops are useful for animations.)
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
        Msg::ChangeColor => {
            model.fill_color = if model.fill_color == Color::A {
                Color::B
            } else {
                Color::A
            };
        }
    }
}

fn draw(canvas: &ElRef<HtmlCanvasElement>, fill_color: Color) {
    let canvas = canvas.get().expect("get canvas element");
    let ctx = seed::canvas_context_2d(&canvas);

    ctx.rect(0., 0., 200., 100.);
    ctx.set_fill_style(&JsValue::from_str(fill_color.as_str()));
    ctx.fill();

    ctx.move_to(0., 0.);
    ctx.line_to(200., 100.);
    ctx.stroke();
}

// `view` describes what to display.
// pub fn view(model: &Model) -> Vec<Node<Msg>> {
pub fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        h2![format!("X: {}, Y: {}", model.point.x, model.point.y)],
        h2![format!("Last key pressed: {}", model.key_code)],
        button![
            ev(Ev::Click, |_| Msg::ToggleWatching),
            if model.event_streams.is_empty() {
                "Start watching"
            } else {
                "Stop watching"
            }
        ],
        div![
            "This is a counter: ",
            C!["counter"],
            button![model.amount, ev(Ev::Click, |_| Msg::Increment)],
        ],
        div![
            style! {St::Display => "flex"},
            canvas![
                el_ref(&model.canvas),
                attrs![
                    At::Width => px(200),
                    At::Height => px(100),
                ],
                style![
                    St::Border => "1px solid black",
                ],
            ],
            button!["Change color", ev(Ev::Click, |_| Msg::ChangeColor)],
        ],
    ]
}
