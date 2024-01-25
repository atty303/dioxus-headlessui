use dioxus::prelude::*;

use dioxus::dioxus_core::AttributeValue;

use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use web_sys::wasm_bindgen::JsCast;
use crate::RenderFn;


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
    on_close: EventHandler<()>,
    render: Option<RenderFn<DialogRenderArgs>>,
    children: Element,
) -> Element {
    let state = use_signal(|| DialogState {
        open: Signal::new(open),
        dialog: Signal::new(None),
    });
    let _ = use_context_provider(|| state);
    // let dialog = use_signal(|| None);

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

    let mut attrs = vec![
        Attribute::new(
            "onmounted",
            AttributeValue::listener(move |e: MountedEvent| {
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
            div {
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
    let state = use_context::<Signal<DialogState>>();

    let mut attrs = Vec::new();

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
#[test]
fn test() {
    let a = "";
    let b = false;
    rsx! {
        div {}
    //     Dialog {
    //         id: a,
    //         open: b,
    //         on_close: |a| {},
    //     }
    };

    // let mut dom = VirtualDom::new(|cx| {
    //     let a = rsx! {
    //         Dialog {
    //             as_element: as_element!("my_element"),
    //             id: "hogehoge",
    //             open: true,
    //             ""
    //         }
    //     };
    //     render! { a }
    // });
    // let a = dom.rebuild();
    // println!("{:?}", a.templates);
}
