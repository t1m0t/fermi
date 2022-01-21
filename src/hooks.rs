use crate::{AtomId, AtomRoot, Readable, Writable};
use dioxus::core::{ScopeId, ScopeState};
use std::{marker::PhantomData, rc::Rc};

// Initializes the atom root and retuns it;
pub fn use_init_atom_root(cx: &ScopeState) -> &Rc<AtomRoot> {
    cx.use_hook(|_| match cx.consume_context::<AtomRoot>() {
        Some(ctx) => ctx,
        None => cx.provide_context(AtomRoot::new(cx.schedule_update_any())),
    })
}

// Returns the atom root, initiaizing it at the root of the app if it does not exist.
pub fn use_atom_root(cx: &ScopeState) -> &Rc<AtomRoot> {
    cx.use_hook(|_| match cx.consume_context::<AtomRoot>() {
        Some(root) => root,
        None => cx.provide_context(AtomRoot::new(cx.schedule_update_any())),
    })
}

pub fn use_read<'a, V: 'static>(cx: &'a ScopeState, f: impl Readable<V>) -> &'a V {
    log::trace!("use_read atom {:?}", f.unique_id());

    let id = f.unique_id();
    let root = use_atom_root(cx);
    let inner = cx.use_hook(|_| UseReadInner {
        value: None,
        root: root.clone(),
        scope_id: cx.scope_id(),
        id,
    });

    let value = inner.root.register(f, cx.scope_id());
    inner.value = Some(value);
    inner.value.as_ref().unwrap()
}

/// Returns the RC of the value directly
pub fn use_read_rc<'a, V: 'static>(cx: &'a ScopeState, f: impl Readable<V>) -> &'a Rc<V> {
    let root = use_atom_root(cx);
    let inner = cx.use_hook(|_| UseReadInner {
        value: None,
        root: root.clone(),
        scope_id: cx.scope_id(),
        id: f.unique_id(),
    });

    let value = inner.root.register(f, cx.scope_id());
    inner.value = Some(value);
    inner.value.as_ref().unwrap()
}

struct UseReadInner<V> {
    root: Rc<AtomRoot>,
    id: AtomId,
    scope_id: ScopeId,
    value: Option<Rc<V>>,
}

impl<V> Drop for UseReadInner<V> {
    fn drop(&mut self) {
        self.root.unsubscribe(self.id, self.scope_id)
    }
}

pub fn use_read_write<V>(cx: &ScopeState, _f: impl Writable<V>) -> UseReadWrite<V> {
    UseReadWrite {
        cx,
        _p: PhantomData,
    }
}

pub struct UseReadWrite<'a, V> {
    cx: &'a ScopeState,
    _p: PhantomData<V>,
}
impl<V> Copy for UseReadWrite<'_, V> {}
impl<'a, V> Clone for UseReadWrite<'a, V> {
    fn clone(&self) -> Self {
        Self {
            cx: self.cx,
            _p: PhantomData,
        }
    }
}

pub fn use_set<'a, T: 'static>(cx: &'a ScopeState, f: impl Writable<T>) -> &'a Rc<dyn Fn(T)> {
    let root = use_atom_root(cx);
    let hook = cx.use_hook(|_| {
        let id = f.unique_id();
        let root = root.clone();
        let root2 = root.clone();
        let setter = Rc::new(move |new| {
            log::trace!("setting new value");
            root2.set(id, new)
        });
        UseSetInner {
            _root: root,
            setter,
        }
    });
    &hook.setter
}
struct UseSetInner<T> {
    _root: Rc<AtomRoot>,
    setter: Rc<dyn Fn(T)>,
}
