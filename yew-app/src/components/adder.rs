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
            <div class={format!("flex gap-3 m-3 items-center {}", self.style().get_class_name())}>
                <button class={"px-3 py-2 bg-sky-500 hover:bg-sky-600 button rounded-md"} onclick={link.callback(|_| Msg::AddOne)}>{ format!("+{}", props.qtty) }</button>
                <button class={"px-3 py-2 bg-green-500 hover:bg-green-600 button rounded-md"} onclick={link.callback(|_| Msg::RemoveOne)}>{ format!("-{}", props.qtty) }</button>
                <p class={"px-3 py-2 rounded-md border-solid border border-slate-200"}>{ self.value }</p>
            </div>
        }
    }
}

impl YieldStyle for Adder {
    fn style_from(&self) -> StyleSource<'static> {
        // language=SCSS prefix={ suffix=}
        css!(
            r#"
                .button {
                    font-weight: 700;
                }
            "#
        )
    }
}
