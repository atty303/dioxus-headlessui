use dioxus::prelude::*;

use crate::components::dialog::use_open_closed::{State, use_open_closed};
use crate::hooks::use_id::use_id;

const DEFAULT_DIALOG_TAG: &str = "div";

enum DialogState {
    Open,
    Closed,
}

#[derive(Props)]
pub struct DialogProps<'a> {
    pub as_element: Option<Template<'static>>,
    pub id: Option<&'a str>,
    pub open: Option<bool>,
    pub on_close: Option<EventHandler<'a, bool>>,
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Dialog<'a>(cx: Scope<'a, DialogProps<'a>>) -> Element<'a> {
    let internal_id = use_id(cx);
    let id = cx.props.id.unwrap_or_else(|| cx.raw_text(format_args!("headlessui-dialog-{}", internal_id.get())));

    let use_open_closed_state = use_open_closed(cx);
    let open = cx.props.open.unwrap_or(use_open_closed_state.is_some_and(|s| s == &State::Open));

    let internal_dialog_ref = use_ref(cx, || None::<VNode>);

    // Validations
    // let has_open = cx.props.open.is_some() || use_open_closed_state.is_some();
    // let has_on_close = cx.props.on_close.is_some();
    // if !has_open && !has_on_close {
    //     panic!("You have to provide an `open` and an `on_close` prop to the `Dialog` component.")
    // }
    // if !has_open {
    //     panic!("You provided an `on_close` prop to the `Dialog` component but forgot an `open` prop.")
    // }
    // if !has_on_close {
    //     panic!("You provided an `open` prop to the `Dialog`, but forgot an `on_close` prop.")
    // }

    let dialog_state = if open { DialogState::Open } else { DialogState::Closed };

    let a = LazyNodes::new(move |__cx: &ScopeState| -> VNode {
        VNode {
            parent: None,
            key: None,
            template: std::cell::Cell::new(cx.props.as_element.unwrap()),
            root_ids: Default::default(),
            dynamic_nodes: __cx.bump().alloc([__cx.make_node(
                rsx! { span { } }
            )]),
            dynamic_attrs: __cx.bump().alloc([]),
        }
    });

    render! { a }
}

macro_rules! as_element {
    ($tag: literal) => {
        Template {
            name: concat!(file!(), ":", line!(), ":", column!(), ":", "0"),
            roots: &[TemplateNode::Element {
                tag: $tag,
                namespace: None,
                attrs: &[],
                children: &[TemplateNode::Dynamic { id: 0usize }],
            }],
            node_paths: &[&[0u8, 0u8]],
            attr_paths: &[],
        }
    };
}

#[cfg(test)]
#[test]
fn test() {
    let a = "";
    let b = false;
    rsx! {
        Dialog {
            id: a,
            open: b,
        }
    };

    let mut dom = VirtualDom::new(|cx| {
        let a = rsx! {
            Dialog {
                as_element: as_element!("my_element"),
                id: "hogehoge",
                open: true,
                ""
            }
        };
        render! { a }
    });
    let a = dom.rebuild();
    println!("{:?}", a.templates);
}

// mod dioxus_elements {
//     use dioxus::prelude::dioxus_elements::*;
//
//     pub struct my_element;
//     impl my_element {
//         pub const TAG_NAME: &'static str = "hogehoge";
//         pub const NAME_SPACE: Option<&'static str> = None;
//     }
// }
//
