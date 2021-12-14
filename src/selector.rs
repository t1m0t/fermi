use crate::{root::AtomRoot, AtomId, Readable};

pub type Selector<O> = fn(AtomRoot) -> O;

impl<V> Readable<V> for fn(AtomRoot) -> V {
    fn read(&self, _root: AtomRoot) -> Option<V> {
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

#[cfg(test)]
mod tests {
    // use dioxus::prelude::Scope;

    // use super::*;
    // use crate::{use_read, Atom};

    // static

    // static User: Atom<Option<()>> = |_| None;
    // static IsLoggedIn: Selector<bool> = |cx| cx.read(User).as_ref().is_some();

    // fn test_select(cx: Scope<()>) {
    //     let is_logged_in = use_read(&cx, IsLoggedIn);
    // }
}
