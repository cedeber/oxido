use stylist::{css, StyleSource, YieldStyle};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub qtty: i32,
}

pub enum Msg {
    AddOne,
    RemoveOne,
}

pub struct Adder {
    value: i32,
}

impl Component for Adder {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();

        match msg {
            Msg::AddOne => {
                self.value += props.qtty;
                true
            }
            Msg::RemoveOne => {
                self.value -= props.qtty;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let props = ctx.props();

        html! {
            <div class={self.style()}>
                <button class={"button"} onclick={link.callback(|_| Msg::AddOne)}>{ format!("+{}", props.qtty) }</button>
                <button onclick={link.callback(|_| Msg::RemoveOne)}>{ format!("-{}", props.qtty) }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

impl YieldStyle for Adder {
    fn style_from(&self) -> StyleSource<'static> {
        // language=SCSS prefix={ suffix=}
        css!(
            r#"
color: red;

.button {
  background-color: hotpink;
  border: 0;
  border-radius: 3px;
  cursor: pointer;
}
"#
        )
    }
}
