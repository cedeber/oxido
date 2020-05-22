use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// `Model` describes our app state.
type Model = i32;

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// `view` describes what to display.
pub fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment)],
    ]
}
