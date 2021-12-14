mod atoms;
mod hooks;
mod root;
mod selector;
mod traits;

pub use atoms::*;
pub use hooks::*;
pub use root::*;
pub use selector::*;
pub use traits::*;

pub mod prelude {
    pub use crate::*;
}

#[cfg(test)]
mod integration_test {
    use super::*;
    use dioxus::prelude::*;

    static Title: Atom<String> = |_| "".to_string();
    static Users: AtomFamily<u32, String> = |_| Default::default();

    fn App(cx: Scope<()>) -> Element {
        let _ = use_init_atom_root(&cx);

        cx.render(rsx!(
            Leaf { id: 0 }
            Leaf { id: 1 }
            Leaf { id: 2 }
        ))
    }

    #[derive(Debug, PartialEq, Props)]
    struct LeafProps {
        id: u32,
    }

    fn Leaf(cx: Scope<LeafProps>) -> Element {
        let user = use_read(&cx, Title);
        let user = use_read(&cx, Users);

        use_coroutine(&cx, || {
            //
            async move {
                //
            }
        });

        rsx!(cx, div {
            button {
                "Start"
                onclick: move |_| {}
            }
            button {
                "Stop"
                onclick: move |_| {}
            }
            button {
                "Reverse"
                onclick: move |_| {}
            }
        })
    }
}
