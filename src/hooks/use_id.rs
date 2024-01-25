use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

static ID: AtomicUsize = AtomicUsize::new(0);

pub fn use_id() -> Signal<usize> {
    use_signal(|| ID.fetch_add(1, Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_id() {
        let dom = VirtualDom::new(|| None);
        let cx = dom.base_scope();

        let id1 = use_id();
        assert_eq!(id1(), 0usize);

        let id2 = use_id();
        assert_eq!(id2(), 1usize);
    }
}