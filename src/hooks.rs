use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use dioxus::core::{ScopeId, ScopeState};

use crate::{AtomId, AtomRoot, Readable, Writable};

// Initializes the atom root and retuns it;
pub fn use_init_atom_root(cx: &ScopeState) -> &Rc<RefCell<AtomRoot>> {
    cx.use_hook(
        |_| {
            if let Some(ctx) = cx.consume_state::<RefCell<AtomRoot>>() {
                // panic!("there should not already be an atoms context");
                ctx
            } else {
                let update_any = cx.schedule_update_any();
                cx.provide_state(RefCell::new(AtomRoot::new(update_any)));
                cx.consume_state::<RefCell<AtomRoot>>().unwrap()
            }
        },
        |f| f,
    )
}

pub fn use_atom_root(cx: &ScopeState) -> &Rc<RefCell<AtomRoot>> {
    cx.use_hook(
        |_| cx.consume_state::<RefCell<AtomRoot>>().unwrap(),
        |val| val,
    )
}

pub fn use_read<'a, V: 'static>(cx: &'a ScopeState, f: impl Readable<V>) -> &'a V {
    let id = f.unique_id();
    cx.use_hook(
        |_| {
            let root = cx.consume_state::<RefCell<AtomRoot>>().unwrap();
            let scope_id = cx.scope_id();
            UseReadInner {
                value: None,
                root,
                id,
                scope_id,
            }
        },
        |inner| {
            let value = inner.root.borrow_mut().register(f, cx.scope_id());
            inner.value = Some(value);
            inner.value.as_ref().unwrap()
        },
    )
}

/// Returns the RC of the value directly
pub fn use_read_rc<'a, V: 'static>(cx: &'a ScopeState, f: impl Readable<V>) -> &'a Rc<V> {
    let id = f.unique_id();
    cx.use_hook(
        |_| {
            let root = cx.consume_state::<RefCell<AtomRoot>>().unwrap();
            let scope_id = cx.scope_id();
            UseReadInner {
                value: None,
                root,
                id,
                scope_id,
            }
        },
        |inner| {
            let value = inner.root.borrow_mut().register(f, cx.scope_id());
            inner.value = Some(value);
            inner.value.as_ref().unwrap()
        },
    )
}

struct UseReadInner<V> {
    root: Rc<RefCell<AtomRoot>>,
    id: AtomId,
    scope_id: ScopeId,
    value: Option<Rc<V>>,
}

impl<V> Drop for UseReadInner<V> {
    fn drop(&mut self) {
        self.root.borrow_mut().unsubscribe(self.id, self.scope_id)
    }
}

pub fn use_read_write<'a, V>(cx: &'a ScopeState, _f: impl Writable<V>) -> UseReadWrite<'a, V> {
    UseReadWrite {
        cx: cx,
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
            cx: self.cx.clone(),
            _p: PhantomData,
        }
    }
}

pub fn use_set<'a, T: 'static>(cx: &'a ScopeState, f: impl Writable<T>) -> &'a Rc<dyn Fn(T)> {
    let root = use_atom_root(cx);
    cx.use_hook(
        |_| {
            let id = f.unique_id();
            let root = root.clone();
            let root2 = root.clone();
            let setter = Rc::new(move |new| root2.borrow_mut().set(id, new));
            UseSetInner {
                _root: root,
                setter,
            }
        },
        |f| {
            //
            &f.setter
        },
    )
}
struct UseSetInner<T> {
    _root: Rc<RefCell<AtomRoot>>,
    setter: Rc<dyn Fn(T)>,
}
