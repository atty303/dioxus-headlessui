use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static ID: AtomicUsize = AtomicUsize::new(0);

fn use_id(cx: &ScopeState) -> &UseState<usize> {
    use_state(cx, || ID.fetch_add(1, Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_id() {
        let mut dom = VirtualDom::new(|_| None);
        let cx = dom.base_scope();

        let id1 = use_id(cx);
        assert_eq!(*id1.get(), 0usize);

        let id2 = use_id(cx);
        assert_eq!(*id2.get(), 1usize);
    }
}