use dioxus::prelude::*;
use fermi::*;

static Title: Atom<String> = |_| "".to_string();
static Users: AtomFamily<u32, String> = |_| Default::default();

fn App(cx: Scope<()>) -> Element {
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
            onclick: move |_| {},
            "Start"
        }
        button {
            onclick: move |_| {},
            "Stop"
        }
        button {
            onclick: move |_| {},
            "Reverse"
        }
    })
}
