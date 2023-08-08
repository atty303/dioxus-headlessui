use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum State {
    Open,
    Closed,
    Closing,
    Opening,
}

pub fn use_open_closed(cx: &ScopeState) -> Option<&State> {
    use_context::<State>(cx)
}