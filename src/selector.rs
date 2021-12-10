use crate::{root::AtomRoot, AtomId, Readable};

pub type Selector<O> = fn(AtomRoot) -> O;

impl<V> Readable<V> for fn(AtomRoot) -> V {
    fn read(&self, root: AtomRoot) -> Option<V> {
        todo!()
    }

    fn init(&self) -> V {
        todo!()
    }

    fn unique_id(&self) -> AtomId {
        todo!()
    }
}

// pub trait Selectable {
//     fn select(&self) -> Selection;
// }

// fn select(&self) -> Selection {
//     todo!()
// }
// impl<V> Selectable for fn(AtomRoot) -> V {
//     fn select(&self) -> Selection {
//         todo!()
//     }
// }

// pub struct Selection;

struct SelectorContext {
    inner: AtomRoot,
}

mod tests {
    use dioxus::prelude::Context;

    use super::*;
    use crate::{use_read, Atom};

    // static

    static User: Atom<Option<()>> = |_| None;
    static IsLoggedIn: Selector<bool> = |cx| cx.read(User).as_ref().is_some();

    fn test_select(cx: Context) {
        let is_logged_in = use_read(cx, IsLoggedIn);
    }
}
