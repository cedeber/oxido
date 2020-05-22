use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    event_streams: Vec<StreamHandle>,
    point: Point,
    key_code: u32,
    amount: i32,
}

#[derive(Default)]
pub struct Point {
    x: i32,
    y: i32,
}

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    Increment,
    ToggleWatching,
    MouseMoved(web_sys::MouseEvent),
    KeyPressed(web_sys::KeyboardEvent),
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
    }
}

// `view` describes what to display.
pub fn view(model: &Model) -> Vec<Node<Msg>> {
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
    ]
}
