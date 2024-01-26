use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use web_sys::HtmlDialogElement;
use web_sys::wasm_bindgen::JsCast;

use crate::components::RenderFn;

struct DialogState {
    open: Signal<bool>,
    dialog: Signal<Option<web_sys::HtmlDialogElement>>,
}

pub struct DialogRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub open: bool,
}

/// The main Dialog component.
#[component]
pub fn Dialog(
    /// Whether the Dialog is open or not.
    open: bool,
    /// Called when the Dialog is dismissed (via outside click of the DialogPanel or by pressing the Escape key). Typically used to close the dialog by setting open to false.
    on_close: Option<EventHandler<()>>,
    render: Option<RenderFn<DialogRenderArgs>>,
    children: Element,
) -> Element {
    let state = use_signal(|| DialogState {
        open: Signal::new(open),
        dialog: Signal::new(None),
    });
    let _ = use_context_provider(|| state);
    let dialog = use_signal(|| None::<HtmlDialogElement>);

    use_effect(move || {
        if let Some(dialog) = state.read().dialog.read().as_ref() {
            if *state.read().open.read() {
                if !dialog.has_attribute("open") {
                    dialog.show_modal().unwrap();
                }
            } else {
                if dialog.has_attribute("open") {
                    dialog.close();
                }
            }
        }
    });

    let attrs = vec![
        Attribute::new(
            "onmounted",
            AttributeValue::listener(move |event: Event<PlatformEventData>| {
                let e: MountedEvent = event.map(|e| e.into());
                let el = e
                    .web_event()
                    .dyn_ref::<web_sys::HtmlDialogElement>()
                    .expect("expecting HtmlDialogElement");
                *state.read().dialog.write() = Some(el.clone());
            }),
            None,
            false,
        ),
        Attribute::new(
            "role",
            AttributeValue::Text("dialog".to_string()),
            None,
            false,
        ),
    ];

    if let Some(render) = render {
        render.call(DialogRenderArgs {
            attrs,
            children,
            open,
        })
    } else {
        rsx! {
            dialog {
                ..attrs,
                {children}
            }
        }
    }
}

pub struct DialogPanelRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
}

/// This indicates the panel of your actual Dialog. Clicking outside of this component will trigger the onClose of the Dialog component.
#[component]
pub fn DialogPanel(render: Option<RenderFn<DialogPanelRenderArgs>>, children: Element) -> Element {
    // let _state = use_context::<Signal<DialogState>>();

    let attrs = Vec::new();

    if let Some(render) = render {
        render.call(DialogPanelRenderArgs { attrs, children })
    } else {
        rsx! {
            div {
                ..attrs,
                {children}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use dioxus::dioxus_core::NoOpMutations;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::window;
    use super::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_dialog() {
        fn app() -> Element {
            rsx! {
                Dialog {
                    open: true,
                    children: rsx! {
                        div {
                            "test"
                        }
                    },
                }
            }
        }

        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .set_inner_html(&"<div id='main'></div>".to_string());

        launch(app);
    }
}