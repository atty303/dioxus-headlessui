use dioxus::prelude::*;

const DEFAULT_DIALOG_TAG: &str = "div";

#[derive(Props)]
pub struct DialogProps<'a> {
    pub open: bool,
    pub on_close: EventHandler<'a, bool>,
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Dialog<'a>(cx: Scope<'a, DialogProps<'a>>) -> Element<'a> {
    render! {
        ""
    }
}

